#pragma once

#include <optional>
#include <string>

namespace backend {

std::optional<std::string> run_luau_code(const std::string& code);

} // namespace backend
