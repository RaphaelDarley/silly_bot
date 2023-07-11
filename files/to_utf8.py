f = open("mobypos.txt", "r", encoding="mac_roman")

lines = [l.replace("\\", "|") for l in f]

out = open("mobypos_utf8.txt", "a", encoding="UTF-8")
out.writelines(lines)
out.close()
