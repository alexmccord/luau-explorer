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

static void report(const Luau::TypeError& e) {
    std::cerr << "TypeError: " << Luau::toString(e) << std::endl;
}

void report(const Luau::CheckResult& result) {
    for (const auto& e : result.errors) {
        if (Luau::get<Luau::SyntaxError>(e))
            continue;
        report(e);
    }
}

struct SingleSourceFileResolver : public Luau::FileResolver {
    std::string code;

    SingleSourceFileResolver(const std::string& code)
        : code(code)
    {
    }

    std::optional<Luau::SourceCode> readSource(const Luau::ModuleName& name) override {
        return Luau::SourceCode{code};
    }
};

struct LuauAnalyzer {
    // Because SingleSourceFileResolver always return the source code, we don't care about ModuleNames.
    // We might eventually support multiple modules, but that's a future thing.
    SingleSourceFileResolver file_resolver;
    Luau::NullConfigResolver config_resolver;
    Luau::Frontend frontend;

    LuauAnalyzer(const std::string& code)
        : file_resolver{code}
        , frontend{&file_resolver, &config_resolver, {/*retainFullTypeGraphs=*/true}}
    {
        Luau::LintOptions lint_options;
        lint_options.warningMask = ~0ull;

        config_resolver.defaultConfig.enabledLint = lint_options;
        config_resolver.defaultConfig.mode = Luau::Mode::Nonstrict;
        config_resolver.defaultConfig.parseOptions = Luau::ParseOptions{true, true, true, true};

        Luau::registerBuiltinTypes(frontend.typeChecker);
        Luau::freeze(frontend.typeChecker.globalTypes);
    }
};

Luau::LintResult lint(const std::string& code) {
    LuauAnalyzer analyzer{code};
    return analyzer.frontend.lint("module");
}

Luau::CheckResult check(const std::string& code) {
    LuauAnalyzer analyzer{code};
    return analyzer.frontend.check("module");
}

} // namespace backend
