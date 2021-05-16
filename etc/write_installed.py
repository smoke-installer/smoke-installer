import sys


def main(name):
    open("/etc/ludum/installed.txt", "w").writelines(name)


if __name__ == '__main__':
    game = sys.argv[1]
    main(game)
