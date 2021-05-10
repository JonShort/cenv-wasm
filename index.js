const { parse_env } = require('./pkg/cenv_wasm');

const mockContents = `
# ++ one ++
# TEST_A=1
# TEST_B=1

# ++ two ++
TEST_A=2
TEST_B=2

# ++ three ++
# TEST_A=3
# TEST_B=3
`;

console.log(parse_env(mockContents, "three"))
