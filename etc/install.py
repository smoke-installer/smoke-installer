import os
import sys

def main(password, zippass, name, link):
    os.system(f'curl {link} > {name}.zip')
    os.system(f'unzip -P {zippass} {name}.zip && rm -rf {name}.zip')
    os.system(f'cd {name}/ && echo {password} | sudo -S sh build.sh')
    os.system(f'rm -rf {name}/')
    open("/etc/smoke_installer/installed.txt", "w+").writelines(f"{name}")

if __name__ == '__main__':
    sudo = sys.argv[1]
    zipp = sys.argv[2]
    game = sys.argv[3]
    url = sys.argv[4]
    main(sudo, zipp, game, url)

