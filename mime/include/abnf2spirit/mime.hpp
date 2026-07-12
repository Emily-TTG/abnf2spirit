// SPDX-License-Identifier: X11
// Copyright (C) 2026 Emily "TTG" Banerjee <prs.ttg+abnf2spirit@pm.me>

#pragma once

#include <expected>
#include <string>
#include <string_view>
#include <unordered_map>

#include <cstddef>

namespace abnf2spirit::mime {
	struct mime {
		std::string type;
		std::unordered_map<std::string, std::string> parameters;
	};

	[[nodiscard]]
	std::expected<mime, std::size_t> parse(std::string_view);
}
