#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    // Zmienne lokalne - konfiguracja
    char user1[50];
    char user2[50];
    char group[50];
    char input[100];
    char confirm[10];
    char cmd[256];
    FILE *fp;

    // ========== KROK 1-2: AKTUALIZACJA I INSTALACJA SAMBY ==========
    printf("[1-2] apt update + install samba\n");
    system("apt update -y");
    system("apt install -y samba");

    // ========== KROK 3: STATUS SMBD ==========
    printf("[3] systemctl status smbd\n");
    system("systemctl status smbd");

    // ========== KROK 4: ZAPORA UFW ==========
    printf("[4] ufw allow Samba + ufw status\n");
    system("ufw enable");
    system("ufw allow 'Samba'");
    system("ufw status");

    // ========== KONFIGURACJA MANUALNA - NA DOLE ==========
    system("clear");
    printf("\n╔════════════════════════════════════════════════════╗\n");
    printf("║        KONFIGURATOR SAMBY - MODUŁ MANUALNY         ║\n");
    printf("╚════════════════════════════════════════════════════╝\n\n");
    
    printf("Wcisni ENTER aby zaakceptowac domyslne wartosci\n");
    printf("Lub wpisz nowa wartosc i wcisni ENTER\n\n");
    
    // Nazwa użytkownika 1
    printf("Uzytkownik 1 [wykladowca]: ");
    fgets(input, sizeof(input), stdin);
    if (input[0] != '\n') {
        input[strcspn(input, "\n")] = 0;
        strcpy(user1, input);
    } else {
        strcpy(user1, "wykladowca");
    }
    
    // Nazwa użytkownika 2
    printf("Uzytkownik 2 [student]: ");
    fgets(input, sizeof(input), stdin);
    if (input[0] != '\n') {
        input[strcspn(input, "\n")] = 0;
        strcpy(user2, input);
    } else {
        strcpy(user2, "student");
    }
    
    // Nazwa grupy
    printf("Grupa [uczelnia]: ");
    fgets(input, sizeof(input), stdin);
    if (input[0] != '\n') {
        input[strcspn(input, "\n")] = 0;
        strcpy(group, input);
    } else {
        strcpy(group, "uczelnia");
    }
    
    // Podsumowanie
    system("clear");
    printf("\n╔════════════════════════════════════════════════════╗\n");
    printf("║           PODSUMOWANIE KONFIGURACJI                ║\n");
    printf("╚════════════════════════════════════════════════════╝\n\n");
    printf("Uzytkownik 1:  %s (chmod: 750)\n", user1);
    printf("Uzytkownik 2:  %s (chmod: 770)\n", user2);
    printf("Grupa:         %s\n\n", group);
    
    printf("Potwierdzasz? [y/n]: ");
    fgets(confirm, sizeof(confirm), stdin);
    
    if (confirm[0] != 'y' && confirm[0] != 'Y') {
        printf("Anulowano.\n\n");
        return 1;
    }
    
    system("clear");

    printf("\n");

    // ========== KROK 5: TWORZENIE KATALOGÓW ==========
    printf("[5] mkdir katalogi\n");
    snprintf(cmd, sizeof(cmd), "mkdir -p /samba/%s", user1);
    system(cmd);
    snprintf(cmd, sizeof(cmd), "mkdir -p /samba/%s", user2);
    system(cmd);

    // ========== KROK 6-7: TWORZENIE UŻYTKOWNIKÓW I GRUPY ==========
    printf("[6-7] groupadd + useradd + usermod\n");
    
    snprintf(cmd, sizeof(cmd), "groupadd %s 2>/dev/null", group);
    system(cmd);
    
    snprintf(cmd, sizeof(cmd), "useradd -M -s /sbin/nologin %s 2>/dev/null", user1);
    system(cmd);
    
    snprintf(cmd, sizeof(cmd), "useradd -M -s /sbin/nologin %s 2>/dev/null", user2);
    system(cmd);
    
    snprintf(cmd, sizeof(cmd), "usermod -aG %s %s", group, user1);
    system(cmd);
    
    snprintf(cmd, sizeof(cmd), "usermod -aG %s %s", group, user2);
    system(cmd);

    // ========== KROK 8: HASŁA SAMBY ==========
    printf("[8] smbpasswd\n");
    snprintf(cmd, sizeof(cmd), "smbpasswd -a %s", user1);
    system(cmd);
    snprintf(cmd, sizeof(cmd), "smbpasswd -a %s", user2);
    system(cmd);

    // ========== KROK 9: USTAWIENIE WŁAŚCICIELA KATALOGÓW ==========
    printf("[9] chown\n");
    snprintf(cmd, sizeof(cmd), "chown %s:%s /samba/%s", user1, group, user1);
    system(cmd);
    
    snprintf(cmd, sizeof(cmd), "chown %s:%s /samba/%s", user2, group, user2);
    system(cmd);

    // ========== KROK 10: USTAWIENIE UPRAWNIEŃ ==========
    printf("[10] chmod 750/770\n");
    snprintf(cmd, sizeof(cmd), "chmod 750 /samba/%s", user1);
    system(cmd);
    
    snprintf(cmd, sizeof(cmd), "chmod 770 /samba/%s", user2);
    system(cmd);

    // ========== KROK 11: BACKUP SMB.CONF ==========
    printf("[11] cp smb.conf.bak\n");
    if (system("cp /etc/samba/smb.conf /etc/samba/smb.conf.bak") != 0) {
        perror("Failed to backup smb.conf");
        return 1;
    }

    // ========== KROK 12: USTAWIENIE SERVER STRING ==========
    printf("[12] server string\n");
    
    fp = fopen("/etc/samba/smb.conf", "a");
    if (fp == NULL) {
        perror("Cannot open smb.conf for appending");
        return 1;
    }
    
    fprintf(fp, "\nserver string = Samba Server\n");

    // ========== KROK 13: SEKCJA [UŻYTKOWNIK 1] ==========
    printf("[13] [%s]\n", user1);
    
    fprintf(fp, "\n[%s]\n", user1);
    fprintf(fp, "path = /samba/%s\n", user1);
    fprintf(fp, "browsable = yes\n");
    fprintf(fp, "read only = no\n");
    fprintf(fp, "valid users = @%s\n", group);

    // ========== KROK 14: SEKCJA [UŻYTKOWNIK 2] ==========
    printf("[14] [%s]\n", user2);
    
    fprintf(fp, "\n[%s]\n", user2);
    fprintf(fp, "path = /samba/%s\n", user2);
    fprintf(fp, "valid users = @%s\n", group);
    fprintf(fp, "read only = no\n");
    fprintf(fp, "force group = %s\n", group);
    fprintf(fp, "create mask = 0770\n");

    fclose(fp);

    // ========== KROK 15: RESTART SMBD ==========
    printf("[15] systemctl restart smbd\n");
    system("systemctl restart smbd");

    // ========== PODSUMOWANIE ==========
    printf("\nGotowe!\n");
    system("clear");

    // ========== NETPLAN ==========
    printf("Chcesz skonfigurować netplan? [y/n]: ");
    fgets(confirm, sizeof(confirm), stdin);
    
    if (confirm[0] == 'y' || confirm[0] == 'Y') {
        char ip_cidr[50];
        char interface[50];
        char dhcp_choice[10];
        FILE *detect_fp;
        
        system("clear");
        printf("[NETPLAN] Konfiguracja sieci\n\n");
        
        // Wykrywanie interfejsu - prefer enp0s3 lub enp0s2
        strcpy(interface, "");
        if (system("test -d /sys/class/net/enp0s3") == 0) {
            strcpy(interface, "enp0s3");
        } else if (system("test -d /sys/class/net/enp0s2") == 0) {
            strcpy(interface, "enp0s2");
        } else {
            detect_fp = popen("ip route | grep default | awk '{print $5}'", "r");
            if (detect_fp != NULL) {
                fgets(interface, sizeof(interface), detect_fp);
                interface[strcspn(interface, "\n")] = 0;
                pclose(detect_fp);
            }
        }
        printf("[NETPLAN] Interfejs: %s\n\n", interface);
        
        printf("Uzyj DHCP? [y/n]: ");
        fgets(dhcp_choice, sizeof(dhcp_choice), stdin);
        
        fp = fopen("/etc/netplan/50-cloud-init.yaml", "w");
        if (fp == NULL) {
            perror("Cannot create netplan config");
            return 1;
        }
        
        if (dhcp_choice[0] == 'y' || dhcp_choice[0] == 'Y') {
            fprintf(fp, "network:\n");
            fprintf(fp, "  version: 2\n");
            fprintf(fp, "  ethernets:\n");
            fprintf(fp, "    %s:\n", interface);
            fprintf(fp, "      dhcp4: true\n");
            printf("[NETPLAN] DHCP4 wlaczony\n");
        } else {
            printf("IP/maska [192.168.1.10/24]: ");
            fgets(input, sizeof(input), stdin);
            if (input[0] != '\n') {
                input[strcspn(input, "\n")] = 0;
                strcpy(ip_cidr, input);
            } else {
                strcpy(ip_cidr, "192.168.1.10/24");
            }
            
            fprintf(fp, "network:\n");
            fprintf(fp, "  version: 2\n");
            fprintf(fp, "  ethernets:\n");
            fprintf(fp, "    %s:\n", interface);
            fprintf(fp, "      dhcp4: false\n");
            fprintf(fp, "      addresses:\n");
            fprintf(fp, "        - address: %s\n", ip_cidr);
            fprintf(fp, "      gateway4: 192.168.1.1\n");
            fprintf(fp, "      nameservers:\n");
            fprintf(fp, "        addresses: [8.8.8.8, 8.8.4.4]\n");
            printf("[NETPLAN] IP: %s\n", ip_cidr);
        }
        
        fclose(fp);
        printf("[NETPLAN] netplan apply\n");
        system("netplan apply");
        system("clear");
    }
    
    // ========== USUWANIE PLIKU ==========
    printf("Usunac plik 'a' z /home/? [y/n]: ");
    fgets(confirm, sizeof(confirm), stdin);
    
    if (confirm[0] == 'y' || confirm[0] == 'Y') {
        system("rm /home/a");
        printf("[OK] Plik usuniety\n");
    }
    
    return 0;
}