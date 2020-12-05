#!/bin/zsh
aoc_request () {
  curl --silent --fail --cookie "session=$SESSION" -L "$@"
  # shellcheck disable=SC2181
  while [ $? -ne 0 ]; do
    printf "AOC session token invalid. Please provide new session token from https://adventofcode.com/auth/github:\n";
    read -r SESSION
    curl --silent --fail --cookie "session=$SESSION" -L "$@"
  done;
  echo "$SESSION" > session;
}

cargo_run () {
  cargo run -q --bin "day$DAY" -- --part "$part" --output-file "outputs/day$DAY-$part.txt"
}

cargo_test () {
   cargo test -q --bin "day$DAY"
}

cargo_test_and_run () {
  if ! cargo_test; then
    printf 'Test failed. Run anyways? (%1$sy%2$s/%1$sN%2$s) ' "$BF" "$RS" "$part";
    read -r run;
    if [ "$run" != 'y' ]; then
        return 1
    fi;
  fi;
  mkdir "outputs" 2> /dev/null;
  if cargo_run; then
    printf 'Got value %1$s%3$s%2$s\n' "$BF" "$RS" "$(<"outputs/day$DAY-$part.txt")";
    return 0
  fi;
  return 1
}


cd "$(dirname "$0")" || exit

if [ -z "$1" ];
then
  printf "What day? ";
  read -r DAY;
else
  DAY=$1;
fi

if ! test -f "src/bin/day$DAY.rs"; then
  gucci -s day="day$DAY" < templates/day.rs.tpl > "src/bin/day$DAY.rs";
fi;

touch session;
SESSION=$(<session);


if ! test -f "inputs/day$DAY.txt"; then
  aoc_request "https://adventofcode.com/2020/day/$DAY/input" -o "inputs/day$DAY.txt";
fi;

touch "test_inputs/day$DAY.txt"

git add "src/bin/day$DAY.rs"
git add "inputs/day$DAY.txt"
git add "test_inputs/day$DAY.txt"

BF=$'\u001b[4m\u001b[1m'
RS=$'\u001b[0m'

mkdir /tmp/aoc 2>/dev/null;
printf 'Choose action: %1$sT%2$sest, %1$sR%2$sun, %1$sS%2$submit, Switch %1$sP%2$sart, %1$sQ%2$suit\n' $BF $RS
quit=0
part=1
while [ "$quit" -eq 0 ]; do
    printf "%d > " $part
    read -r action
    action=$(echo "$action" | tr '[:lower:]' '[:upper:]')
    if [ "$action" = 'T' ]; then
      if cargo_test; then
        printf 'Enter %1$sR%2$s to run program\n' $BF $RS;
      fi;
    elif [ "$action" = 'R' ]; then
      if cargo_test_and_run; then
        printf 'Enter %1$sS%2$s to submit this value for part %3$d\n' $BF $RS "$part";
      fi;
    elif [ "$action" = 'S' ]; then
      if ! test -f "outputs/day$DAY-$part.txt"; then
        printf 'Could not find output, running...'
        if cargo_test_and_run; then
          printf 'Submit part %3$d? (%1$sY%2$s/%1$sn%2$s) ' $BF $RS "$part";
          read -r submit;
          if [ "$submit" = 'n' ]; then
              continue;
          fi;
        else continue ;
        fi;
      fi;
      result_file=$(mktemp -p /tmp/aoc "result-$DAY-$part.XXXX.html");
      aoc_request -X POST --data-raw "level=$part&answer=$(<"outputs/day$DAY-$part.txt")" -H 'Content-Type: application/x-www-form-urlencoded' -o "$result_file" "https://adventofcode.com/2020/day/$DAY/answer";
      lynx "$result_file";
    elif [ "$action" = 'P' ]; then
      if [ "$part" -eq 2 ]; then
        part=1;
      else
        part=2;
      fi;
      echo "Part is now $part";
    elif [ "$action" = 'Q' ]; then
      quit=1;
    fi;
done
