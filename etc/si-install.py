import getpass
import os
import sys
import tkinter


def start_install(zipname, zippass, game):
    password = inputProjectBox.get("1.0", "end-1c")
    os.chdir(f"/home/{getpass.getuser()}/SmokeInstaller")
    os.system(f'curl https://smoke-installer.github.io/{zipname} > {game}.zip')
    os.system(f'unzip -P {zippass} {zipname}')
    os.chdir(f"{game}/")
    os.system(f'echo {password} | sudo -S sh build.sh')
    os.chdir(f"/home/{getpass.getuser()}/SmokeInstaller")
    os.remove(f"{game}/*")
    os.system(f'echo {password} | sudo -S echo "{game}" > /etc/smoke_installer/installed.txt')

if __name__ == '__main__':
    zipurl = sys.argv[1]
    zippass = sys.argv[2]
    gamename = sys.argv[3]

    root = tkinter.Tk()
    root.title("Install")

    TT = tkinter.Label(root, text="Enter password to install:")
    TT.config(font=("Courier", 14))
    TT.pack()

    inputProjectBox = tkinter.Text(root, height=2, width=10)
    inputProjectBox.pack()

    enterButton = tkinter.Button(root, height=1, width=10, text="Enter", command=lambda: start_install(zipurl, zippass, gamename))
    enterButton.pack()

    root.geometry("500x500")
    root.mainloop()

