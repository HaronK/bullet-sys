
if(MSVC)
    add_definitions(-DWIN32)
endif(MSVC)

subdirs(../bullet3)
