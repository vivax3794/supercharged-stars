cp update_template.json update.json
sed -i -e s/VERSION/${1}/g update.json
darwin="$(curl https://github.com/vivax3794/supercharged-stars/releases/download/v${1}/supercharged-stars.app.tar.gz.sig -L)"
linux="$(curl https://github.com/vivax3794/supercharged-stars/releases/download/v${1}/supercharged-stars_${1}_amd64.AppImage.tar.gz.sig -L)"
windows="$(curl https://github.com/vivax3794/supercharged-stars/releases/download/v${1}/supercharged-stars_${1}_x64_en-US.msi.zip.sig -L)"

sed -i -e "s/DARWIN_SIG/${darwin}/g" update.json
sed -i -e "s/LINUX_SIG/${linux}/g" update.json
sed -i -e "s/WINDOWS_SIG/${windows}/g" update.json