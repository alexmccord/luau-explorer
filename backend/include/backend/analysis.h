#pragma once

#include <optional>
#include <string>

#include <Luau/Frontend.h>

namespace backend {

void report(const Luau::LintResult& result);

struct SingleSourceFileResolver : public Luau::FileResolver {
    SingleSourceFileResolver(const std::string& code);

    std::string code;

    std::optional<Luau::SourceCode> readSource(const Luau::ModuleName& name) override;
};

Luau::LintResult lint(const std::string& code);

} // namespace backend