#include <iostream>

#include <Luau/BuiltinDefinitions.h>

#include "backend/analysis.h"

namespace backend {

static void report(const Luau::LintWarning& w) {
    std::cerr << Luau::LintWarning::getName(w.code) << ": " << w.text << std::endl;
}

void report(const Luau::LintResult& result) {
    for (const auto& e : result.errors)
        report(e);

    for (const auto& w : result.warnings)
        report(w);
}

SingleSourceFileResolver::SingleSourceFileResolver(const std::string& code)
    : code(code)
{
}

std::optional<Luau::SourceCode> SingleSourceFileResolver::readSource(const Luau::ModuleName& name)  {
    return Luau::SourceCode{code};
}

Luau::LintResult lint(const std::string& code) {
    SingleSourceFileResolver file_resolver{code};
    Luau::NullConfigResolver config_resolver;
    Luau::FrontendOptions frontend_options{/*retainFullTypeGraphs=*/true};
    Luau::Frontend frontend{&file_resolver, &config_resolver, frontend_options};
    Luau::registerBuiltinTypes(frontend.typeChecker);

    // Really doesn't matter what we call it. SingleSourceFileResolver always return the source code.
    // We might eventually support multiple modules, but that's a future thing.
    Luau::LintOptions lint_options;
    lint_options.warningMask = ~0ull;
    return frontend.lint("module", lint_options);
}

} // namespace backend
