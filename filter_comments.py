import sys
import os

dirin = sys.argv[1]
dirout = sys.argv[2]

os.makedirs(dirout, exist_ok=True)

def exec(cmd):
    p = os.popen(cmd)
    for l in p:
        print(l.rstrip())
    if p.close():
        print(f"\"{cmd}\" did not exit succesfully")
        sys.exit(1)

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
                    continue
                comment = line.find('//')
                if comment >= 0:
                    line = line[:comment].rstrip()
                if len(line):
                    print(line, file=fo)

    exec(f"rustfmt {fno}")
