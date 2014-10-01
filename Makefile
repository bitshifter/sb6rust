MEDIA_ZIP_URL = http://openglsuperbible.com/files/sb6media_2013_11_10.zip
MEDIA_ZIP_FILE = sb6media_2013_11_10.zip
MEDIA_ZIP_DIR = .tmp
MEDIA_ZIP_PATH = $(MEDIA_ZIP_DIR)/$(MEDIA_ZIP_FILE)
MEDIA_ZIP_MD5 = 8a1d75203d601f9a4b98befc02a0b987

all: build copyshaders copymedia

build:
	cargo build

copyshaders: target/media/shaders

copymedia: target/media/textures target/media/objects

downloadmedia: $(MEDIA_ZIP_PATH)

update:
	cargo update

clean:
	cargo clean

target/media/shaders:
	@mkdir -p target/media
	cp -ru shaders target/media

target/media/textures target/media/objects: $(MEDIA_ZIP_PATH)
	@mkdir -p target/media
	unzip -n $(MEDIA_ZIP_PATH) -d target/media

$(MEDIA_ZIP_PATH):
	mkdir $(MEDIA_ZIP_DIR)
	curl -o $(MEDIA_ZIP_PATH) $(MEDIA_ZIP_URL)
	echo "$(MEDIA_ZIP_MD5) $(MEDIA_ZIP_PATH)" | md5sum -c -

.PHONY: \
	all \
	build \
	clean \
	copymedia \
	copyshaders \
	downloadmedia \
	update
