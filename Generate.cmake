# SPDX-License-Identifier: X11
# Copyright (C) 2026 Emily "TTG" Banerjee <prs.ttg+abnf2spirit@pm.me>

find_program(ABNF2SPIRIT_CARGO_PROGRAM cargo REQUIRED)

function(abnf2spirit_generate_grammar _ABNF _TEMPLATE _OUTPUT)
	message(STATUS "Generating Spirit.X4 grammar from '${_ABNF}'")

	execute_process(
			COMMAND ${CMAKE_COMMAND} -E env
			${ABNF2SPIRIT_CARGO_PROGRAM} run
			--release
			--manifest-path ${CMAKE_CURRENT_FUNCTION_LIST_DIR}/Cargo.toml
			--
			${_ABNF}
			${_TEMPLATE}
			${_OUTPUT}
			WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}
			COMMAND_ECHO STDOUT
			ENVIRONMENT CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR}
			RESULT_VARIABLE _GENERATE_RESULT)

	if(NOT _GENERATE_RESULT EQUAL 0)
		message(
				FATAL_ERROR
				"grammar generation failed: ${_GENERATE_RESULT}")
	endif()
endfunction()
