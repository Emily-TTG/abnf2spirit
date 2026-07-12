// SPDX-License-Identifier: X11
// Copyright (C) 2026 Emily "TTG" Banerjee <prs.ttg+abnf2spirit@pm.me>

#include <ranges>
#include <vector>

#include <abnf2spirit/mime.hpp>
#include <abnf2spirit/mime.inl>

#include <boost/spirit/x4/parse.hpp>

#include <boost/fusion/include/adapt_struct.hpp>

namespace abnf2spirit::mime_detail {
	using range = std::ranges::subrange<std::string_view::const_iterator>;

	struct parameter {
		range name;
		range value;
	};

	struct content {
		range type;
		std::vector<parameter> parameters;
	};
}

BOOST_FUSION_ADAPT_STRUCT(abnf2spirit::mime_detail::parameter, name, value)
BOOST_FUSION_ADAPT_STRUCT(abnf2spirit::mime_detail::content, type, parameters)

namespace abnf2spirit {
	const auto content =
			boost::spirit::x4::omit[boost::spirit::x4::no_case[boost::spirit::x4::lit("Content-Type")] >> ':' >> mime_grammar::OWS] >>
			boost::spirit::x4::raw[mime_grammar::type >> '/' >> mime_grammar::subtype] >>
			*(boost::spirit::x4::omit[mime_grammar::OWS >> ';' >> mime_grammar::OWS] >>
					boost::spirit::x4::as<mime_detail::parameter>(
							boost::spirit::x4::raw[mime_grammar::regular_parameter_name] >>
							boost::spirit::x4::omit['='] >>
							boost::spirit::x4::raw[mime_grammar::value]));

	std::expected<mime::parsed, std::size_t> parse(const std::string_view string) {
		mime_detail::content captured;

		const auto result = boost::spirit::x4::parse(
				string,
				content,
				captured);

		if(!result.completed() && !result.ok) {
			// TODO(Emily): Wire in expectation points so this is actually
			//              useful.
			return std::unexpected(
					string.size() - result.remainder_str().size());
		}

		mime::parsed out{};
		out.type.assign(captured.type.begin(), captured.type.end());
		for(const auto& [ name, value ] : captured.parameters) {
			out.parameters.emplace(
					std::string(name.begin(), name.end()),
					std::string(value.begin(), value.end()));
		}

		return out;
	}
}
