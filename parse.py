
def run():
    with open("mem-stats.txt", "r") as f:
        lines = f.readlines()
    data = [line.strip().split(" ") for line in lines[1::2]]
    data = [[d[0], d[-1]] for d in data]
    # print(data[:10])
    # print(len(data))
   
    # mem_percent = [int(d[0]) for d in data]
    # print(mem_percent[-10:])
    
    for d in data:
        print(int(d[1]) * 1.0 / 100000)
    



if __name__ == "__main__":
    run()