#include "backend/json.h"

#include <Luau/JsonEncoder.h>
#include <Luau/Parser.h>

namespace backend {

std::string jsonify(const std::string& code) {
    Luau::Allocator allocator;
    Luau::AstNameTable name_table{allocator};
    Luau::ParseOptions options{true, true, true, true};

    Luau::ParseResult result = Luau::Parser::parse(code.data(), code.size(), name_table, allocator, options);
    return Luau::toJson(result.root);
}

} // namespace backend
