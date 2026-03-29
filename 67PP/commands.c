#include <stdio.h>

int main(void) {
    printf("1.apt update -y\n
            2.apt instal samba\n
            3.systemctl status smbd\n
            4.ufw allow 'Samba'/ufw status\n
            5.mkdir -p /samba/wykladowca//student\n
            6.useradd -M -s/sbin/nologin wykladowca//student\n
            7.usermod -aG uczelnia wykladowca//student\n
            8.smbpasswd -a wykladowca//student\n
            9.chown wykladowca:uczelnia /samba/wykladowca//student\n
            10.chmod 750//770 /samba/wykladowca//student\n
            11.cp /etc/samba/smb.conf , -||- .bak, nano -||-\n
            12. server sting = Samba Server\n
            13.[Wykladowca], path = /samba/wykladowca, browsable = yes, \nread only = no, valid users = @uczelnia\n
            14.[Student], path = /samba/student/, valid users = @uczelnia,\n read only = no, force group = uczelnia, create mask = 0770
            15.systemctl restart smbd\n");
    return 0;
}