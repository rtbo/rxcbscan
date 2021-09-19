set -e

cargo run

python filter_comments.py gen/main/ffi gen/main.nocmt/ffi
python filter_comments.py gen/main gen/main.nocmt
python filter_comments.py gen/current/ffi gen/current.nocmt/ffi
python filter_comments.py gen/current gen/current.nocmt

python test_diff.py gen/current.nocmt/ffi gen/main.nocmt/ffi
python test_diff.py gen/current.nocmt gen/main.nocmt
