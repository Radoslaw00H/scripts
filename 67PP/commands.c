#include <stdio.h>

int main(void) {
    printf(
        "1. apt update -y\n"
        "2. apt install samba\n"
        "3. systemctl status smbd\n"
        "4. ufw allow 'Samba' / ufw status\n"
        "5. mkdir -p /samba/wykladowca /samba/student\n"
        "6. useradd -M -s /sbin/nologin wykladowca\n"
        "   useradd -M -s /sbin/nologin student\n"
        "7. usermod -aG uczelnia wykladowca\n"
        "   usermod -aG uczelnia student\n"
        "8. smbpasswd -a wykladowca\n"
        "   smbpasswd -a student\n"
        "9. chown wykladowca:uczelnia /samba/wykladowca\n"
        "   chown student:uczelnia /samba/student\n"
        "10. chmod 750 /samba/wykladowca\n"
        "    chmod 770 /samba/student\n"
        "11. cp /etc/samba/smb.conf /etc/samba/smb.conf.bak\n"
        "    nano /etc/samba/smb.conf\n"
        "12. server string = Samba Server\n"
        "13. [Wykladowca]\n"
        "    path = /samba/wykladowca\n"
        "    browsable = yes\n"
        "    read only = no\n"
        "    valid users = @uczelnia\n"
        "14. [Student]\n"
        "    path = /samba/student\n"
        "    valid users = @uczelnia\n"
        "    read only = no\n"
        "    force group = uczelnia\n"
        "    create mask = 0770\n"
        "15. systemctl restart smbd\n"
    );

    return 0;
}