#pragma once

#include <optional>
#include <string>

#include <Luau/Frontend.h>

namespace backend {

void report(const Luau::LintResult& result);
void report(const Luau::CheckResult& result);

Luau::LintResult lint(const std::string& code);
Luau::CheckResult check(const std::string& code);

std::string hydrate(const std::string& code);

} // namespace backend
