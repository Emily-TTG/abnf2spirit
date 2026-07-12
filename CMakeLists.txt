# SPDX-License-Identifier: X11
# Copyright (C) 2026 Emily "TTG" Banerjee <prs.ttg+abnf2spirit@pm.me>

cmake_minimum_required(VERSION 4.4)

project(abnf2spirit CXX)

include(${CMAKE_CURRENT_LIST_DIR}/Generate.cmake)

set(
		ABNF2SPIRIT_MIME_GRAMMAR_DIR
		${CMAKE_CURRENT_BINARY_DIR}/include/abnf2spirit/)

set(ABNF2SPIRIT_MIME_GRAMMAR ${ABNF2SPIRIT_MIME_GRAMMAR_DIR}/mime.inl)

file(MAKE_DIRECTORY ${ABNF2SPIRIT_MIME_GRAMMAR_DIR})
abnf2spirit_generate_grammar(
		${CMAKE_CURRENT_LIST_DIR}/mime/mime.abnf
		${CMAKE_CURRENT_LIST_DIR}/mime/mime.inl.in
		${ABNF2SPIRIT_MIME_GRAMMAR})

add_library(abnf2spirit_mime STATIC)
add_library(abnf2spirit::mime ALIAS abnf2spirit_mime)

target_compile_features(abnf2spirit_mime PUBLIC cxx_std_20)

target_link_libraries(abnf2spirit_mime PRIVATE Boost::fusion Boost::spirit_x4)

target_include_directories(
		abnf2spirit_mime PRIVATE
		${ABNF2SPIRIT_MIME_GRAMMAR_DIR})

target_include_directories(
		abnf2spirit_mime PUBLIC
		${CMAKE_CURRENT_LIST_DIR}/mime/include)

target_sources(
		abnf2spirit_mime PRIVATE
		${CMAKE_CURRENT_LIST_DIR}/mime/mime.cpp
		${MIME_GRAMMAR})

target_sources(
		abnf2spirit_mime PUBLIC
		${CMAKE_CURRENT_LIST_DIR}/mime/include/abnf2spirit/mime.hpp)
