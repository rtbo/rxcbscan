import sys
import os

dirin = sys.argv[1]
dirout = sys.argv[2]

os.makedirs(dirout, exist_ok=True)

for f in os.listdir(dirin):
    fni = os.path.join(dirin, f)
    if os.path.isdir(fni):
        continue
    fno = os.path.join(dirout, f)

    with open(fni, "r") as fi:
        with open(fno, "w") as fo:
            for line in fi:
                line = line.rstrip()
                if len(line) == 0:
                    print("", file=fo)
                    continue
                comment = line.find('//')
                if comment >= 0:
                    line = line[:comment].rstrip()
                if len(line):
                    print(line, file=fo)

    os.popen(f"rustfmt {fno}")
