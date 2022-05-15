mkdir ~/src/turing
cd ~/src/turing
curl https://github.com/Uncodeable864/turing/releases/download/v1.0.0/turing_apple_x86_64_darwin -L -o turing
# echo  >> ~/.zshrc
echo "export PATH=\"\$PATH:\$HOME/src/turing\"" >> ~/.zshrc
echo "Your password is needed to install turing"
sudo chmod +x turing
echo "When you run turing, Gatekeeper will detect it (this is becuase I don't have a Apple Developer Account.. yet). You need to goto System Preferences > Security & Privacy > Allow All (near bottom of page)."
echo "Now, restart your Terminal and have fun!"