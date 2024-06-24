cargo build --release


DIR="$HOME/.local/rand-emoji"

mkdir -p "$DIR"

cp target/release/rand-emoji "$DIR"
cp emojis.txt "$DIR"
chmod +x "$DIR/rand-emoji"

OMZ_DIR="$HOME/.oh-my-zsh"

if [ -d "$OMZ_DIR" ]; then
	cp refined-emoji.zsh-theme "$OMZ_DIR/custom/themes"
	
	REGEX='^ZSH_THEME=".*"$'
	REPLACEMENT='ZSH_THEME="refined-emoji"'
	sed -i.bak -E "s/$REGEX/$REPLACEMENT/" "$HOME/.zshrc"
else
	echo "Didn't find Oh My Zsh. If it's installed then this script will set a custom theme for it with emoji."
fi




