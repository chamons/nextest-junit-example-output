# Nextest JUnit Errors Example

This is an example repo that has a failing test, a passing test, and a flaky test.

It can generate a junit output file which can be processed by https://github.com/chamons/circleci-junit-fix

`% cargo nextest run --profile ci`