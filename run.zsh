#!/bin/zsh

export LYNX_CFG=".lynx/lynx.cfg"

BF=$'\u001b[4m\u001b[1m'
RS=$'\u001b[0m'

lynx_scroll () {
  lynx -cmd_script .lynx/scroll.script "$@"
}

aoc_request () {
  while ! curl --silent --fail --cookie .lynx/cookie -L "$@"; do
    printf 'Not logged in. Visit https://adventofcode.com/auth/login. %1$sO%2$spen in terminal or launch %1$sB%2$srowser? ' $BF $RS;
    read -r open_key;
    if [ "$open_key" = 'O' ]; then
      lynx_scroll https://adventofcode.com/auth/login;
    else
      if [ "$open_key" = 'B' ]; then
        xdg-open https://adventofcode.com/auth/login;
      fi;
      printf 'Session token: '
      read -r token;
      printf "adventofcode.com\tFALSE\t/\tTRUE\t1922612654\tsession\t%s\n" "$token" > .lynx/cookie;
    fi;
  done;
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

if ! test -f "inputs/day$DAY.txt"; then
  aoc_request "https://adventofcode.com/2020/day/$DAY/input" -o "inputs/day$DAY.txt";
fi;

touch "test_inputs/day$DAY.txt"

git add "src/bin/day$DAY.rs"
git add "inputs/day$DAY.txt"
git add "test_inputs/day$DAY.txt"



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
      lynx_scroll "$result_file";
      if grep -q "That's the right answer\!"; then
        if [ "$part" -eq 1 ]; then
          printf 'Continue to part 2? (%1$sY%2$s/%1$sn%2$s) ' $BF $RS;
          read -r cont;
          if [ "$cont" != 'n' ]; then
            printf 'To open part 2 in the terminal enter %1$sO%2$s. To open part 2 in your browser enter %1$sB%2$s' $BF $RS;
          fi;
        else
          printf 'To show leaderboard enter %1$sL%2$s. To quit enter %1$sQ%2$s' $BF $RS;
        fi;
      else
        printf 'Enter %1$sS%2$s to try again\n' $BF $RS "$part";
      fi;
    elif [ "$action" = 'P' ]; then
      if [ "$part" -eq 2 ]; then
        part=1;
      else
        part=2;
      fi;
      echo "Part is now $part";
    elif [ "$action" = 'O' ]; then
      lynx_scroll "https://adventofcode.com/2020/day/$DAY";
    elif [ "$action" = 'B' ]; then
      xdg-open "https://adventofcode.com/2020/day/$DAY";
    elif [ "$action" = 'Q' ]; then
      quit=1;
    fi;
done
