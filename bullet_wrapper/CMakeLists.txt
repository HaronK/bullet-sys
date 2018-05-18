cmake_minimum_required(VERSION 2.4.3)

# TODO: there should be more correct way to add this define using cmake-rs
if(MSVC)
    add_definitions(-DWIN32)
endif(MSVC)

subdirs(../bullet)
