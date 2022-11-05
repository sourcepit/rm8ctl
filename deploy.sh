#!/bin/bash
set -e

TARGET_HOST=pi@192.168.43.31

APP_DIR="/opt/rm8"
APP_NAME="rm8ctl"
APP_FILE_NAME="$APP_NAME"
APP_FILE="$APP_DIR/$APP_FILE_NAME"

CONF_DIR="/etc/rm8"
CONF_NAME=$APP_NAME
CONF_FILE_NAME="$CONF_NAME.conf"
CONF_FILE="$CONF_DIR/$CONF_FILE_NAME"

scp "target/armv7-unknown-linux-gnueabihf/release/$APP_FILE_NAME" $TARGET_HOST:/tmp/$APP_FILE_NAME

ssh $TARGET_HOST -T /bin/bash << EOF
	#!/bin/bash
	set -e
	sudo su -
	
	# deploy application
	mkdir -p $APP_DIR
	rm -f $APP_DIR/$APP_FILE_NAME
	mv /tmp/$APP_FILE_NAME $APP_DIR
	chown root:root $APP_DIR/$APP_FILE_NAME
	chmod a+x $APP_DIR/$APP_FILE_NAME
EOF
	