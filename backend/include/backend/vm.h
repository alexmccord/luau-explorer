#pragma once

#include <optional>
#include <string>

namespace backend {

std::optional<std::string> run(const std::string& code);

} // namespace backend
