use cc;
use std::path::Path;
use walkdir::WalkDir;

trait BuildExt {
    fn all_files<P: AsRef<Path>>(&mut self, p: P, ext: &str) -> &mut Self;
}

impl BuildExt for cc::Build {
    fn all_files<P: AsRef<Path>>(&mut self, p: P, ext: &str) -> &mut Self {
        for entry in WalkDir::new(p).into_iter().filter_map(|e| e.ok()) {
            if let Some(ext_str) = entry.path().extension() {
                if ext_str == ext {
                    self.file(entry.path());
                }
            }
        }

        self
    }
}

pub fn build(with_examples: bool) {
    build_extras();
    build_src();

    if with_examples {
        build_examples();
    }
}

fn build_extras() {
    cc::Build::new()
        .include("bullet3/src")
        .define("BulletInverseDynamicsUtils_EXPORTS", None)
        .define("BT_USE_DOUBLE_PRECISION", None)
        .define("NDEBUG", None)
        .define("USE_GRAPHICAL_BENCHMARK", None)
        .opt_level(3)
        .cpp(true)
        .flag("-fkeep-inline-functions")
        .warnings(false)
        .all_files("bullet3/Extras/InverseDynamics", "cpp")
        .compile("BulletInverseDynamicsUtils");

    cc::Build::new()
        .include("bullet3/src")
        .include("bullet3/Extras/Serialize/BulletFileLoader")
        .define("BulletFileLoader_EXPORTS", None)
        .define("BT_USE_DOUBLE_PRECISION", None)
        .define("NDEBUG", None)
        .define("USE_GRAPHICAL_BENCHMARK", None)
        .opt_level(3)
        .cpp(true)
        .flag("-fkeep-inline-functions")
        .warnings(false)
        .all_files("bullet3/Extras/Serialize/BulletFileLoader", "cpp")
        .compile("BulletFileLoader");

    cc::Build::new()
        .include("bullet3/src")
        .define("BulletWorldImporter_EXPORTS", None)
        .define("BT_USE_DOUBLE_PRECISION", None)
        .define("NDEBUG", None)
        .define("USE_GRAPHICAL_BENCHMARK", None)
        .opt_level(3)
        .cpp(true)
        .flag("-fkeep-inline-functions")
        .warnings(false)
        .all_files("bullet3/Extras/Serialize/BulletWorldImporter", "cpp")
        .compile("BulletWorldImporter");
}

fn build_src() {
    cc::Build::new()
        .include("bullet3/src")
        .define("BT_USE_DOUBLE_PRECISION", None)
        .define("LinearMath_EXPORTS", None)
        .define("NDEBUG", None)
        .define("USE_GRAPHICAL_BENCHMARK", None)
        .opt_level(3) // ignoring OPT_LEVEL from the crate
        .cpp(true)
        .flag("-fkeep-inline-functions")
        .warnings(false)

        .all_files("bullet3/src/LinearMath", "cpp")

        .compile("LinearMath");

    cc::Build::new()
        .include("bullet3/src")
        .define("BT_USE_DOUBLE_PRECISION", None)
        .define("BulletCollision_EXPORTS", None)
        .define("NDEBUG", None)
        .define("USE_GRAPHICAL_BENCHMARK", None)
        .opt_level(3)
        .cpp(true)
        .flag("-fkeep-inline-functions")
        .warnings(false)
        .all_files("bullet3/src/BulletCollision", "cpp")
        .compile("BulletCollision");

    cc::Build::new()
        .include("bullet3/src")
        .define("BT_USE_DOUBLE_PRECISION", None)
        .define("BulletDynamics_EXPORTS", None)
        .define("NDEBUG", None)
        .define("USE_GRAPHICAL_BENCHMARK", None)
        .opt_level(3)
        .cpp(true)
        .flag("-fkeep-inline-functions")
        .warnings(false)
        .all_files("bullet3/src/BulletDynamics", "cpp")
        .compile("BulletDynamics");

    cc::Build::new()
        .include("bullet3/src")
        .define("Bullet3Common_EXPORTS", None)
        .define("NDEBUG", None)
        .define("USE_GRAPHICAL_BENCHMARK", None)
        .define("BT_USE_DOUBLE_PRECISION", None)
        .opt_level(3)
        .cpp(true)
        .flag("-fkeep-inline-functions")
        .warnings(false)
        .all_files("bullet3/src/Bullet3Common", "cpp")
        .compile("Bullet3Common");

    cc::Build::new()
        .include("bullet3/src")
        .define("BulletInverseDynamics_EXPORTS", None)
        .define("BT_USE_DOUBLE_PRECISION", None)
        .define("NDEBUG", None)
        .define("USE_GRAPHICAL_BENCHMARK", None)
        .opt_level(3)
        .cpp(true)
        .flag("-fkeep-inline-functions")
        .warnings(false)
        .all_files("bullet3/src/BulletInverseDynamics", "cpp")
        .compile("BulletInverseDynamics");
}

fn build_examples() {
    cc::Build::new()
        .include("bullet3/src")
        .include("examples/OpenGLWindow/..")
        .include("bullet3/examples/OpenGLWindow/../ThirdPartyLibs")
        .include("bullet3/examples/OpenGLWindow/../../src")
        .include("bullet3/examples/ThirdPartyLibs/Glew")
        .include("bullet3/examples/ThirdPartyLibs/optionalX11")
        .define("BT_USE_DOUBLE_PRECISION", None)
        .define("DYNAMIC_LOAD_X11_FUNCTIONS", Some("1"))
        .define("GLEW_DYNAMIC_LOAD_ALL_GLX_FUNCTIONS", Some("1"))
        .define("GLEW_STATIC", None)
        .define("GLEW_INIT_OPENGL11_FUNCTIONS", Some("1"))
        .define("OpenGLWindow_EXPORTS", None)
        .define("NDEBUG", None)
        .define("USE_GRAPHICAL_BENCHMARK", None)
        .opt_level(3)
        .cpp(true)
        .flag("-fkeep-inline-functions")
        .warnings(false)

        .file("bullet3/examples/OpenGLWindow/X11OpenGLWindow.cpp")
        .file("bullet3/examples/ThirdPartyLibs/Glew/glew.c")

        .all_files("bullet3/examples/OpenGLWindow", "cpp")

        // .file("bullet3/examples/OpenGLWindow/EGLOpenGLWindow.cpp")
        // .file("bullet3/examples/OpenGLWindow/GLFWOpenGLWindow.cpp")
        // .file("bullet3/examples/OpenGLWindow/GLInstancingRenderer.cpp")
        // .file("bullet3/examples/OpenGLWindow/GLPrimitiveRenderer.cpp")
        // .file("bullet3/examples/OpenGLWindow/GLRenderToTexture.cpp")
        // .file("bullet3/examples/OpenGLWindow/LoadShader.cpp")
        // .file("bullet3/examples/OpenGLWindow/OpenSans.cpp")
        // .file("bullet3/examples/OpenGLWindow/SimpleOpenGL2App.cpp")
        // .file("bullet3/examples/OpenGLWindow/SimpleOpenGL2Renderer.cpp")
        // .file("bullet3/examples/OpenGLWindow/SimpleOpenGL3App.cpp")
        // .file("bullet3/examples/OpenGLWindow/TwFonts.cpp")
        // .file("bullet3/examples/OpenGLWindow/fontstash.cpp")
        // .file("bullet3/examples/OpenGLWindow/opengl_fontstashcallbacks.cpp")
        .compile("OpenGLWindow");

    cc::Build::new()
        .include("bullet3/src")
        .include("bullet3/examples/ExampleBrowser/")
        .include("bullet3/examples/ThirdPartyLibs")
        .include("bullet3/examples/ThirdPartyLibs/Glew")
        .define("BulletExampleBrowserLib_EXPORTS", None)
        .define("BT_USE_DOUBLE_PRECISION", None)
        .define("NDEBUG", None)
        .define("USE_GRAPHICAL_BENCHMARK", None)
        .define("GLEW_DYNAMIC_LOAD_ALL_GLX_FUNCTIONS", Some("1"))
        .define("GLEW_INIT_OPENGL11_FUNCTIONS", Some("1"))
        .define("GLEW_STATIC", None)
        .define("GWEN_COMPILE_STATIC", None)
        .define("_STATIC_CPPLIB", None)
        .opt_level(3)
        .cpp(true)
        .flag("-fkeep-inline-functions")
        .warnings(false)

        .all_files("bullet3/examples/Utils", "cpp")
        .all_files("bullet3/examples/ExampleBrowser", "cpp")

        // .file("bullet3/examples/Utils/b3Clock.cpp")
        // .file("bullet3/examples/Utils/ChromeTraceUtil.cpp")
        // .file("bullet3/examples/Utils/b3ResourcePath.cpp")
        // .file("bullet3/examples/ExampleBrowser/OpenGLExampleBrowser.cpp")
        // .file("bullet3/examples/ExampleBrowser/OpenGLGuiHelper.cpp")
        // .file("bullet3/examples/ExampleBrowser/GL_ShapeDrawer.cpp")
        // .file("bullet3/examples/ExampleBrowser/CollisionShape2TriangleMesh.cpp")
        // .file("bullet3/examples/ExampleBrowser/GwenGUISupport/GraphingTexture.cpp")
        // .file("bullet3/examples/ExampleBrowser/GwenGUISupport/GwenParameterInterface.cpp")
        // .file("bullet3/examples/ExampleBrowser/GwenGUISupport/GwenProfileWindow.cpp")
        // .file("bullet3/examples/ExampleBrowser/GwenGUISupport/GwenTextureWindow.cpp")
        // .file("bullet3/examples/ExampleBrowser/GwenGUISupport/gwenUserInterface.cpp")
        .compile("BulletExampleBrowserLib");

    cc::Build::new()
        .include("bullet3/src")
        .include("bullet3/examples/ThirdPartyLibs")
        .include("bullet3/examples/ThirdPartyLibs/Glew")
        .include("bullet3/examples/ThirdPartyLibs/optionalX11")
        .define("DYNAMIC_LOAD_X11_FUNCTIONS", Some("1"))
        .define("GLEW_DYNAMIC_LOAD_ALL_GLX_FUNCTIONS", Some("1"))
        .define("GLEW_INIT_OPENGL11_FUNCTIONS", Some("1"))
        .define("GLEW_STATIC", None)
        .define("GWEN_COMPILE_STATIC", None)
        .define("_STATIC_CPPLIB", None)
        .define("gwen_EXPORTS", None)
        .define("NDEBUG", None)
        .define("USE_GRAPHICAL_BENCHMARK", None)
        .define("BT_USE_DOUBLE_PRECISION", None)
        .opt_level(3)
        .cpp(true)
        .flag("-fkeep-inline-functions")
        .warnings(false)

        .all_files("bullet3/examples/ThirdPartyLibs/Gwen", "cpp")

        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Anim.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/BaseRender.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/DragAndDrop.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Gwen.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Hook.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Skin.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/ToolTip.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Utility.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/events.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/inputhandler.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/Base.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/Button.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/Canvas.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/CheckBox.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/ColorControls.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/ColorPicker.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/ComboBox.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/CrossSplitter.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/DockBase.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/DockedTabControl.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/Dragger.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/GroupBox.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/HSVColorPicker.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/HorizontalScrollBar.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/HorizontalSlider.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/ImagePanel.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/Label.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/LabelClickable.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/ListBox.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/Menu.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/MenuItem.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/MenuStrip.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/NumericUpDown.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/PanelListPanel.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/ProgressBar.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/Properties.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/RadioButton.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/RadioButtonController.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/ResizableControl.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/Resizer.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/RichLabel.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/ScrollBar.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/ScrollBarBar.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/ScrollBarButton.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/ScrollControl.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/Slider.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/SplitterBar.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/TabButton.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/TabControl.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/TabStrip.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/Text.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/TextBox.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/TextBoxNumeric.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/TreeControl.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/TreeNode.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/VerticalScrollBar.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/VerticalSlider.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/WindowControl.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/Dialog/FileOpen.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/Dialog/FileSave.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Controls/Dialog/Query.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Platforms/Null.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Platforms/Windows.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/Gwen/Renderers/OpenGL_DebugFont.cpp")
        .compile("gwen");

    cc::Build::new()
        .include("bullet3/src")
        .define("BussIK_EXPORTS", None)
        .define("NDEBUG", None)
        .define("USE_GRAPHICAL_BENCHMARK", None)
        .define("BT_USE_DOUBLE_PRECISION", None)
        .opt_level(3)
        .cpp(true)
        .flag("-fkeep-inline-functions")
        .warnings(false)

        .all_files("bullet3/examples/ThirdPartyLibs/BussIK", "cpp")

        // .file("bullet3/examples/ThirdPartyLibs/BussIK/Jacobian.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/BussIK/LinearR2.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/BussIK/LinearR3.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/BussIK/LinearR4.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/BussIK/MatrixRmn.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/BussIK/Misc.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/BussIK/Node.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/BussIK/Tree.cpp")
        // .file("bullet3/examples/ThirdPartyLibs/BussIK/VectorRn.cpp")
        .compile("BussIK");

    cc::Build::new()
        .include("bullet3/src")
        .include("bullet3/examples/ThirdPartyLibs")
        .include("bullet3/examples/ThirdPartyLibs/enet/include")
        .include("bullet3/examples/ThirdPartyLibs/clsocket/src")
        .define("BT_ENABLE_CLSOCKET", None)
        .define("BT_ENABLE_ENET", None)
        .define("HAS_SOCKLEN_T", None)
        .define("_LINUX", None)
        .define("BT_USE_DOUBLE_PRECISION", None)
        .define("libpybullet_EXPORTS", None)
        .define("NDEBUG", None)
        .define("USE_GRAPHICAL_BENCHMARK", None)
        .opt_level(3)
        .cpp(true)
        .flag("-fkeep-inline-functions")
        .warnings(false)
        .file("bullet3/examples/ExampleBrowser/InProcessExampleBrowser.cpp")
        .file("bullet3/examples/OpenGLWindow/SimpleCamera.cpp")
        .file("bullet3/examples/TinyRenderer/geometry.cpp")
        .file("bullet3/examples/TinyRenderer/model.cpp")
        .file("bullet3/examples/TinyRenderer/tgaimage.cpp")
        .file("bullet3/examples/TinyRenderer/our_gl.cpp")
        .file("bullet3/examples/TinyRenderer/TinyRenderer.cpp")
        .file("bullet3/examples/SharedMemory/TinyRendererVisualShapeConverter.cpp")
        .file("bullet3/examples/SharedMemory/IKTrajectoryHelper.cpp")
        .file("bullet3/examples/SharedMemory/InProcessMemory.cpp")
        .file("bullet3/examples/SharedMemory/PhysicsClient.cpp")
        .file("bullet3/examples/SharedMemory/PhysicsServer.cpp")
        .file("bullet3/examples/SharedMemory/PhysicsServerExample.cpp")
        .file("bullet3/examples/SharedMemory/PhysicsServerExampleBullet2.cpp")
        .file("bullet3/examples/SharedMemory/SharedMemoryInProcessPhysicsC_API.cpp")
        .file("bullet3/examples/SharedMemory/PhysicsServerSharedMemory.cpp")
        .file("bullet3/examples/SharedMemory/PhysicsDirect.cpp")
        .file("bullet3/examples/SharedMemory/PhysicsDirectC_API.cpp")
        .file("bullet3/examples/SharedMemory/PhysicsServerCommandProcessor.cpp")
        .file("bullet3/examples/SharedMemory/b3PluginManager.cpp")
        .file("bullet3/examples/SharedMemory/PhysicsClientSharedMemory.cpp")
        .file("bullet3/examples/SharedMemory/PhysicsClientSharedMemory_C_API.cpp")
        .file("bullet3/examples/SharedMemory/PhysicsClientC_API.cpp")
        .file("bullet3/examples/SharedMemory/Win32SharedMemory.cpp")
        .file("bullet3/examples/SharedMemory/PosixSharedMemory.cpp")
        .file("bullet3/examples/Utils/RobotLoggingUtil.cpp")
        .file("bullet3/examples/ThirdPartyLibs/tinyxml/tinystr.cpp")
        .file("bullet3/examples/ThirdPartyLibs/tinyxml/tinyxml.cpp")
        .file("bullet3/examples/ThirdPartyLibs/tinyxml/tinyxmlerror.cpp")
        .file("bullet3/examples/ThirdPartyLibs/tinyxml/tinyxmlparser.cpp")
        .file("bullet3/examples/ThirdPartyLibs/Wavefront/tiny_obj_loader.cpp")
        .file("bullet3/examples/ThirdPartyLibs/stb_image/stb_image.cpp")
        .file("bullet3/examples/Importers/ImportColladaDemo/LoadMeshFromCollada.cpp")
        .file("bullet3/examples/Importers/ImportObjDemo/LoadMeshFromObj.cpp")
        .file("bullet3/examples/Importers/ImportObjDemo/Wavefront2GLInstanceGraphicsShape.cpp")
        .file("bullet3/examples/Importers/ImportMJCFDemo/BulletMJCFImporter.cpp")
        .file("bullet3/examples/Importers/ImportURDFDemo/BulletUrdfImporter.cpp")
        .file("bullet3/examples/Importers/ImportURDFDemo/MyMultiBodyCreator.cpp")
        .file("bullet3/examples/Importers/ImportURDFDemo/URDF2Bullet.cpp")
        .file("bullet3/examples/Importers/ImportURDFDemo/UrdfParser.cpp")
        .file("bullet3/examples/Importers/ImportURDFDemo/urdfStringSplit.cpp")
        .file("bullet3/examples/Importers/ImportMeshUtility/b3ImportMeshUtility.cpp")
        .file("bullet3/examples/MultiThreading/b3PosixThreadSupport.cpp")
        .file("bullet3/examples/MultiThreading/b3Win32ThreadSupport.cpp")
        .file("bullet3/examples/MultiThreading/b3ThreadSupportInterface.cpp")
        .file("bullet3/examples/SharedMemory/PhysicsClientUDP.cpp")
        .file("bullet3/examples/SharedMemory/PhysicsClientUDP_C_API.cpp")
        .file("bullet3/examples/ThirdPartyLibs/enet/win32.c")
        .file("bullet3/examples/ThirdPartyLibs/enet/unix.c")
        .file("bullet3/examples/ThirdPartyLibs/enet/callbacks.c")
        .file("bullet3/examples/ThirdPartyLibs/enet/compress.c")
        .file("bullet3/examples/ThirdPartyLibs/enet/host.c")
        .file("bullet3/examples/ThirdPartyLibs/enet/list.c")
        .file("bullet3/examples/ThirdPartyLibs/enet/packet.c")
        .file("bullet3/examples/ThirdPartyLibs/enet/peer.c")
        .file("bullet3/examples/ThirdPartyLibs/enet/protocol.c")
        .file("bullet3/examples/SharedMemory/PhysicsClientTCP.cpp")
        .file("bullet3/examples/SharedMemory/PhysicsClientTCP_C_API.cpp")
        .file("bullet3/examples/ThirdPartyLibs/clsocket/src/SimpleSocket.cpp")
        .file("bullet3/examples/ThirdPartyLibs/clsocket/src/ActiveSocket.cpp")
        .file("bullet3/examples/ThirdPartyLibs/clsocket/src/PassiveSocket.cpp")
        .compile("pybullet");
}
