echo "$0"
cd "$(dirname "$0")" || exit



gucci -s day="$1" < templates/day.rs.tpl > "src/bin/$1.rs"
touch "inputs/$1.txt"
touch "test_inputs/$1.txt"
git add "src/bin/$1.rs"
git add "inputs/$1.txt"
git add "test_inputs/$1.txt"