set -e

cargo run

python filter_comments.py gen/master/ffi gen/master.nocmt/ffi
python filter_comments.py gen/master gen/master.nocmt
python filter_comments.py gen/current/ffi gen/current.nocmt/ffi
python filter_comments.py gen/current gen/current.nocmt

python test_diff.py gen/current.nocmt/ffi gen/master.nocmt/ffi
python test_diff.py gen/current.nocmt gen/master.nocmt
