f = open("mobypos.txt", "r", encoding="mac_roman")
lines = [l.rstrip().split("\\") for l in f]
nouns = [l[0] for l in lines if l[1] == "N"]

out_lines = [f"{n.lower()}\n" for n in nouns]

out = open("nouns_lower.txt", "a", encoding="UTF-8")
out.writelines(out_lines)
out.close()
