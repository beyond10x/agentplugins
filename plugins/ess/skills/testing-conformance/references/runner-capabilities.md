# Verified runner compatibility snapshot

ESS release [0.52.0](https://github.com/beyond10x/ess/tree/0.52.0) generates Go and TypeScript
runners that admit conformance suites through `ess-conformance/27`. The old `/21` ceiling is
obsolete. Admission includes the deletion, absent-input and presence suite families; it does not
prove a particular target implements them. This is a dated compatibility exception: read the
selected producer's source and generated README before assuming the same range in another release.

Reproduce against the implementation's module and its selected specification:

```console
ess specify toolchain which
ess verify conform synthesize --path <specification> --target go --out <module-root>
ess verify conform synthesize --path <specification> --target typescript --out <typescript-output>
```

Run `ESS_REPORT_FORMAT=2 go test ./...` in the Go module and the TypeScript package's documented
command in its generated project, also selecting report/2. Inspect the suite's version and the
actual runner's admission check, and retain the report's executed and skipped counts. Exercise a
supported suite and a controlled incorrect response that must fail. Higher suites may require
another supported runner or an external scenario-status report; select that only when its contract
is verified. An unsupported format, absent runtime, missing fixture provider or undecidable result
is an explicit evidence gap, not a passing trial.
