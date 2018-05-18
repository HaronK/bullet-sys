cmake_minimum_required(VERSION 2.4.3)

if(MSVC)
    add_definitions(-DWIN32)
endif(MSVC)

subdirs(../bullet3)
