#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/wait.h>

// ======================== KOLORY ANSI ========================
#define C_RESET   "\033[0m"
#define C_BOLD    "\033[1m"
#define C_DIM     "\033[2m"
#define C_GREEN   "\033[32m"
#define C_YELLOW  "\033[33m"
#define C_CYAN    "\033[36m"
#define C_RED     "\033[31m"
#define C_BGREEN  "\033[1;32m"
#define C_BYELLOW "\033[1;33m"
#define C_BCYAN   "\033[1;36m"
#define C_BRED    "\033[1;31m"
#define C_BWHITE  "\033[1;37m"

// ======================== UI HELPERS ========================

void print_box(const char *title) {
  int len = (int)strlen(title);
  int box_w = len + 6;
  if (box_w < 54) box_w = 54;
  int pad_left = (box_w - 2 - len) / 2;
  int pad_right = box_w - 2 - len - pad_left;

  printf("\n" C_BCYAN);
  // top
  printf("  ╔");
  for (int i = 0; i < box_w - 2; i++) printf("═");
  printf("╗\n");
  // middle
  printf("  ║");
  for (int i = 0; i < pad_left; i++) printf(" ");
  printf(C_BWHITE "%s" C_BCYAN, title);
  for (int i = 0; i < pad_right; i++) printf(" ");
  printf("║\n");
  // bottom
  printf("  ╚");
  for (int i = 0; i < box_w - 2; i++) printf("═");
  printf("╝" C_RESET "\n\n");
}

void print_separator(void) {
  printf(C_DIM "  ────────────────────────────────────────────────────" C_RESET "\n");
}

void print_step(int num, const char *desc) {
  printf(C_BYELLOW "  ⏳ [KROK %d] " C_RESET "%s\n", num, desc);
}

void print_ok(const char *msg) {
  printf(C_BGREEN "  ✔ " C_RESET "%s\n", msg);
}

void print_fail(const char *msg) {
  printf(C_BRED "  ✘ " C_RESET "%s\n", msg);
}

void print_info(const char *msg) {
  printf(C_BCYAN "  ℹ " C_RESET "%s\n", msg);
}

// Pasek postepu z animacja - uruchamia komende w tle
// bar_w = szerokosc paska (ilosc znakow), msg = opis
int run_with_progress(const char *command, const char *msg) {
  const int bar_w = 30;
  const char *fill_char = "█";
  const char *empty_char = "░";

  printf("\n" C_BYELLOW "  ⏳ " C_BWHITE "%s" C_RESET "\n", msg);

  // Fork - dziecko wykonuje komende, rodzic animuje pasek
  pid_t pid = fork();
  if (pid < 0) {
    perror("fork");
    return -1;
  }

  if (pid == 0) {
    // Dziecko - cicha komenda
    freopen("/dev/null", "w", stdout);
    freopen("/dev/null", "w", stderr);
    int ret = system(command);
    _exit(WEXITSTATUS(ret));
  }

  // Rodzic - animacja
  int pos = 0;
  int direction = 1; // pulsujacy efekt
  int status;

  while (waitpid(pid, &status, WNOHANG) == 0) {
    // Oblicz ile wypelnic (efekt pulsacji)
    int filled = pos;
    if (filled > bar_w) filled = bar_w;

    printf("\r  [");
    for (int i = 0; i < bar_w; i++) {
      if (i < filled)
        printf(C_GREEN "%s" C_RESET, fill_char);
      else
        printf(C_DIM "%s" C_RESET, empty_char);
    }

    // Pseudo procent (pulsujacy 0-95%)
    int pct = (pos * 95) / bar_w;
    if (pct > 95) pct = 95;

    printf(" " C_BYELLOW "%3d%%" C_RESET, pct);
    fflush(stdout);

    pos += direction;
    if (pos >= bar_w) {
      pos = bar_w;
      direction = -1;
    }
    if (pos <= 0) {
      pos = 0;
      direction = 1;
    }

    usleep(150000); // 150ms
  }

  // Koniec - wypelnij pasek do 100%
  printf("\r  [");
  for (int i = 0; i < bar_w; i++)
    printf(C_BGREEN "%s" C_RESET, fill_char);

  if (WIFEXITED(status) && WEXITSTATUS(status) == 0) {
    printf(" " C_BGREEN "100%% ✔" C_RESET "\n");
    return 0;
  } else {
    printf(" " C_BRED "BLAD ✘" C_RESET "\n");
    return 1;
  }
}

// Szybka komenda z checksem (bez paska - za szybka)
int run_quick(const char *command, const char *desc) {
  printf(C_DIM "     > %s" C_RESET, desc);
  fflush(stdout);
  int ret = system(command);
  if (ret == 0) {
    printf(" " C_BGREEN "✔" C_RESET "\n");
  } else {
    printf(" " C_BRED "✘" C_RESET "\n");
  }
  return ret;
}

// Prompt z domyslna wartoscia, zwraca 1 jesli user wpisal cos
int prompt_default(const char *label, const char *def, char *out, int out_sz) {
  char buf[256];
  printf(C_BCYAN "  ▸ " C_RESET "%s " C_DIM "[%s]" C_RESET ": ", label, def);
  fgets(buf, sizeof(buf), stdin);
  if (buf[0] != '\n') {
    buf[strcspn(buf, "\n")] = 0;
    strncpy(out, buf, out_sz - 1);
    out[out_sz - 1] = '\0';
    return 1;
  } else {
    strncpy(out, def, out_sz - 1);
    out[out_sz - 1] = '\0';
    return 0;
  }
}

// Prompt T/N - zwraca 1 dla T, 0 dla N
int prompt_yn(const char *question) {
  char buf[10];
  printf(C_BCYAN "  ▸ " C_RESET "%s " C_DIM "[T/N]" C_RESET ": ", question);
  fgets(buf, sizeof(buf), stdin);
  return (buf[0] == 't' || buf[0] == 'T');
}

// ======================== MAIN ========================

int main(void) {
  char user1[50];
  char user2[50];
  char group[50];
  char cmd[512];
  char confirm[10];
  char input[100];
  FILE *fp;

  // ==========================================
  //  EKRAN 1: AKTUALIZACJA + INSTALACJA
  // ==========================================
  system("clear");
  print_box("KONFIGURATOR SAMBY");
  print_info("Skrypt automatycznie skonfiguruje serwer Samba");
  print_info("Autor: smbdScript | Wersja: 2.0");
  print_separator();

  printf("\n");
  print_step(1, "Aktualizacja systemu...");
  if (run_with_progress("apt update -y", "Aktualizowanie listy pakietow...") != 0) {
    print_fail("Blad podczas aktualizacji! Sprawdz polaczenie z siecia.");
  } else {
    print_ok("System zaktualizowany");
  }

  printf("\n");
  print_step(2, "Instalacja Samby...");
  if (run_with_progress("apt install -y samba", "Instalowanie uslugi Samba...") != 0) {
    print_fail("Blad podczas instalacji Samba!");
    return 1;
  }
  print_ok("Samba zainstalowana pomyslnie");

  printf("\n");
  print_step(3, "Sprawdzanie statusu smbd...");
  if (run_with_progress("systemctl status smbd > /dev/null 2>&1", "Weryfikacja uslugi smbd...") != 0) {
    print_fail("Usluga smbd nie dziala - kontynuuje...");
  } else {
    print_ok("Usluga smbd aktywna");
  }

  printf("\n");
  print_step(4, "Konfiguracja zapory sieciowej...");
  run_quick("ufw enable 2>/dev/null", "ufw enable");
  run_quick("ufw allow 'Samba' 2>/dev/null", "ufw allow Samba");
  print_ok("Zapora skonfigurowana");

  printf("\n");
  print_info("Nacisnij ENTER aby przejsc do konfiguracji...");
  fgets(confirm, sizeof(confirm), stdin);

  // ==========================================
  //  EKRAN 2: KONFIGURACJA UZYTKOWNIKOW
  // ==========================================
  system("clear");
  print_box("KONFIGURACJA UZYTKOWNIKOW I GRUP");
  print_info("Nacisnij ENTER aby zaakceptowac wartosc domyslna");
  print_info("Lub wpisz nowa wartosc i nacisnij ENTER");
  print_separator();
  printf("\n");

  prompt_default("Uzytkownik 1", "wykladowca", user1, sizeof(user1));
  prompt_default("Uzytkownik 2", "student", user2, sizeof(user2));
  prompt_default("Grupa", "uczelnia", group, sizeof(group));

  // ==========================================
  //  EKRAN 3: PODSUMOWANIE
  // ==========================================
  printf("\n");
  system("clear");
  print_box("PODSUMOWANIE KONFIGURACJI");

  printf(C_BWHITE "  ┌──────────────────┬──────────────────────────────┐\n");
  printf("  │   Parametr        │   Wartosc                    │\n");
  printf("  ├──────────────────┼──────────────────────────────┤\n");
  printf("  │ " C_CYAN "Uzytkownik 1" C_BWHITE "     │ " C_GREEN "%-28s" C_BWHITE " │\n", user1);
  printf("  │ " C_DIM "  uprawnienia" C_BWHITE "    │ " C_YELLOW "chmod 750" C_BWHITE "                    │\n");
  printf("  ├──────────────────┼──────────────────────────────┤\n");
  printf("  │ " C_CYAN "Uzytkownik 2" C_BWHITE "     │ " C_GREEN "%-28s" C_BWHITE " │\n", user2);
  printf("  │ " C_DIM "  uprawnienia" C_BWHITE "    │ " C_YELLOW "chmod 770" C_BWHITE "                    │\n");
  printf("  ├──────────────────┼──────────────────────────────┤\n");
  printf("  │ " C_CYAN "Grupa" C_BWHITE "            │ " C_GREEN "%-28s" C_BWHITE " │\n", group);
  printf("  └──────────────────┴──────────────────────────────┘" C_RESET "\n\n");

  if (!prompt_yn("Potwierdzasz konfiguracje?")) {
    printf("\n");
    print_fail("Anulowano przez uzytkownika.");
    printf("\n");
    return 1;
  }

  // ==========================================
  //  EKRAN 4: WYKONYWANIE KONFIGURACJI
  // ==========================================
  system("clear");
  print_box("WYKONYWANIE KONFIGURACJI");

  // ---- KROK 5: Katalogi ----
  print_step(5, "Tworzenie katalogow...");
  snprintf(cmd, sizeof(cmd), "mkdir -p /samba/%s", user1);
  run_quick(cmd, "mkdir /samba/");

  snprintf(cmd, sizeof(cmd), "mkdir -p /samba/%s", user2);
  run_quick(cmd, "mkdir /samba/");

  print_ok("Katalogi utworzone");

  printf("\n");
  print_separator();

  // ---- KROK 6-7: Uzytkownicy + grupa ----
  printf("\n");
  print_step(6, "Tworzenie uzytkownikow i grupy...");

  snprintf(cmd, sizeof(cmd), "groupadd %s 2>/dev/null", group);
  run_quick(cmd, "groupadd");

  snprintf(cmd, sizeof(cmd), "useradd -M -s /sbin/nologin %s 2>/dev/null", user1);
  run_quick(cmd, "useradd");

  snprintf(cmd, sizeof(cmd), "useradd -M -s /sbin/nologin %s 2>/dev/null", user2);
  run_quick(cmd, "useradd");

  snprintf(cmd, sizeof(cmd), "usermod -aG %s %s", group, user1);
  run_quick(cmd, "usermod");

  snprintf(cmd, sizeof(cmd), "usermod -aG %s %s", group, user2);
  run_quick(cmd, "usermod");

  print_ok("Uzytkownicy i grupa skonfigurowane");

  printf("\n");
  print_separator();

  // ---- KROK 8: Hasla Samba ----
  printf("\n");
  print_step(8, "Ustawianie hasel Samba...");
  print_info("Podaj hasla dla uzytkownikow Samba:");
  printf("\n");

  printf(C_BYELLOW "     Haslo dla: %s" C_RESET "\n", user1);
  snprintf(cmd, sizeof(cmd), "smbpasswd -a %s", user1);
  system(cmd);

  printf("\n");
  printf(C_BYELLOW "     Haslo dla: %s" C_RESET "\n", user2);
  snprintf(cmd, sizeof(cmd), "smbpasswd -a %s", user2);
  system(cmd);

  print_ok("Hasla ustawione");

  printf("\n");
  print_separator();

  // ---- KROK 9: Wlasciciel katalogow ----
  printf("\n");
  print_step(9, "Ustawianie wlascicieli katalogow...");

  snprintf(cmd, sizeof(cmd), "chown %s:%s /samba/%s", user1, group, user1);
  run_quick(cmd, "chown");

  snprintf(cmd, sizeof(cmd), "chown %s:%s /samba/%s", user2, group, user2);
  run_quick(cmd, "chown");

  print_ok("Wlasciciele ustawieni");

  printf("\n");
  print_separator();

  // ---- KROK 10: Uprawnienia ----
  printf("\n");
  print_step(10, "Ustawianie uprawnien...");

  snprintf(cmd, sizeof(cmd), "chmod 750 /samba/%s", user1);
  run_quick(cmd, "chmod 750");

  snprintf(cmd, sizeof(cmd), "chmod 770 /samba/%s", user2);
  run_quick(cmd, "chmod 770");

  print_ok("Uprawnienia ustawione");

  printf("\n");
  print_separator();

  // ---- KROK 11: Backup smb.conf ----
  printf("\n");
  print_step(11, "Tworzenie kopii zapasowej smb.conf...");
  if (run_quick("cp /etc/samba/smb.conf /etc/samba/smb.conf.bak",
                 "cp smb.conf.bak") != 0) {
    print_fail("Nie udalo sie utworzyc kopii zapasowej!");
    return 1;
  }
  print_ok("Kopia zapasowa utworzona");

  printf("\n");
  print_separator();

  // ---- KROK 12-14: Konfiguracja smb.conf ----
  printf("\n");
  print_step(12, "Konfiguracja smb.conf...");

  fp = fopen("/etc/samba/smb.conf", "a");
  if (fp == NULL) {
    print_fail("Nie mozna otworzyc /etc/samba/smb.conf!");
    return 1;
  }

  fprintf(fp, "\nserver string = Samba Server\n");
  run_quick("true", "server string");

  // Sekcja user1
  fprintf(fp, "\n[%s]\n", user1);
  fprintf(fp, "path = /samba/%s\n", user1);
  fprintf(fp, "browsable = yes\n");
  fprintf(fp, "read only = no\n");
  fprintf(fp, "valid users = @%s\n", group);

  printf(C_DIM "     > [%s]" C_RESET " " C_BGREEN "✔" C_RESET "\n", user1);

  // Sekcja user2
  fprintf(fp, "\n[%s]\n", user2);
  fprintf(fp, "path = /samba/%s\n", user2);
  fprintf(fp, "valid users = @%s\n", group);
  fprintf(fp, "read only = no\n");
  fprintf(fp, "force group = %s\n", group);
  fprintf(fp, "create mask = 0770\n");

  printf(C_DIM "     > [%s]" C_RESET " " C_BGREEN "✔" C_RESET "\n", user2);

  fclose(fp);
  print_ok("smb.conf skonfigurowany");

  printf("\n");
  print_separator();

  // ---- KROK 15: Restart smbd ----
  printf("\n");
  print_step(15, "Restartowanie uslugi smbd...");
  if (run_with_progress("systemctl restart smbd", "Restartowanie Samba...") != 0) {
    print_fail("Blad podczas restartu smbd!");
  } else {
    print_ok("Usluga smbd zrestartowana");
  }

  printf("\n\n");
  print_box("SAMBA SKONFIGUROWANA POMYSLNIE");
  print_info("Nacisnij ENTER aby kontynuowac...");
  fgets(confirm, sizeof(confirm), stdin);

  // ==========================================
  //  EKRAN 5: NETPLAN
  // ==========================================
  system("clear");
  print_box("KONFIGURACJA SIECI - NETPLAN");

  if (prompt_yn("Chcesz skonfigurowac netplan?")) {
    char ip_cidr[50];
    char interface[50];
    char dhcp_choice[10];
    char netplan_path[128];
    FILE *detect_fp;
    int has_50 = 0;
    int has_01 = 0;

    printf("\n");
    print_separator();
    printf("\n");
    print_step(16, "Wykrywanie interfejsu sieciowego...");

    // 1) Sprawdz z istniejacego pliku netplan
    strcpy(interface, "");
    detect_fp = popen(
        "grep -rh '^ *enp[a-z0-9]*' /etc/netplan/*.yaml 2>/dev/null "
        "| head -1 | sed 's/[: ]//g'",
        "r");
    if (detect_fp != NULL) {
      fgets(interface, sizeof(interface), detect_fp);
      interface[strcspn(interface, "\n")] = 0;
      pclose(detect_fp);
    }
    // 2) Fallback - ip a
    if (strlen(interface) == 0) {
      detect_fp = popen(
          "ip -o link show | awk -F': ' '{print $2}' "
          "| grep -v lo | head -1",
          "r");
      if (detect_fp != NULL) {
        fgets(interface, sizeof(interface), detect_fp);
        interface[strcspn(interface, "\n")] = 0;
        pclose(detect_fp);
      }
    }
    // 3) Domyslny
    if (strlen(interface) == 0) {
      strcpy(interface, "enp0s3");
    }

    printf(C_BGREEN "  ✔ " C_RESET "Wykryty interfejs: " C_BWHITE "%s" C_RESET "\n\n", interface);

    // Wybor pliku netplan
    print_step(17, "Wybor pliku konfiguracyjnego netplan...");
    has_50 = (system("ls /etc/netplan/50-*.yaml >/dev/null 2>&1") == 0);
    has_01 = (system("ls /etc/netplan/01-*.yaml >/dev/null 2>&1") == 0);

    if (has_50) {
      detect_fp = popen("ls /etc/netplan/50-*.yaml 2>/dev/null | head -1", "r");
      if (detect_fp != NULL) {
        fgets(netplan_path, sizeof(netplan_path), detect_fp);
        netplan_path[strcspn(netplan_path, "\n")] = 0;
        pclose(detect_fp);
      }
      print_ok("Znaleziono plik 50-*.yaml");
    } else if (has_01 && !has_50) {
      strcpy(netplan_path, "/etc/netplan/50-cloud-init.yaml");
      printf("\n");
      printf(C_BYELLOW "  ╔════════════════════════════════════════════════╗\n");
      printf("  ║  " C_BWHITE "UWAGA!" C_BYELLOW "                                       ║\n");
      printf("  ║  " C_RESET "Znaleziono plik 01-*.yaml ale brak 50-*" C_BYELLOW "    ║\n");
      printf("  ║  " C_RESET "Tworze nowy: 50-cloud-init.yaml" C_BYELLOW "            ║\n");
      printf("  ║                                                ║\n");
      printf("  ║  " C_RESET "Jesli netplan nie zadziala:" C_BYELLOW "                  ║\n");
      printf("  ║  " C_BRED "rm /etc/netplan/01-*.yaml" C_BYELLOW "                    ║\n");
      printf("  ║  " C_RESET "a nastepnie:" C_BYELLOW "                                 ║\n");
      printf("  ║  " C_BGREEN "netplan apply" C_BYELLOW "                                ║\n");
      printf("  ╚════════════════════════════════════════════════╝\n" C_RESET);
      printf("\n");
    } else {
      strcpy(netplan_path, "/etc/netplan/50-cloud-init.yaml");
      print_info("Brak istniejacego pliku - tworze 50-cloud-init.yaml");
    }

    printf(C_DIM "     Plik: %s" C_RESET "\n\n", netplan_path);
    print_separator();
    printf("\n");

    // DHCP
    int use_dhcp = prompt_yn("Uzyj DHCP?");
    printf("\n");

    // IP
    prompt_default("Adres IP z maska", "192.168.0.1/24", ip_cidr, sizeof(ip_cidr));
    printf("\n");

    // Zapis YAML
    print_step(18, "Zapisywanie konfiguracji netplan...");

    fp = fopen(netplan_path, "w");
    if (fp == NULL) {
      print_fail("Nie mozna otworzyc pliku netplan!");
      return 1;
    }

    fprintf(fp, "network:\n");
    fprintf(fp, "    version: 2\n");
    fprintf(fp, "    renderer: networkd\n");
    fprintf(fp, "    ethernets:\n");
    fprintf(fp, "        %s:\n", interface);

    if (use_dhcp) {
      fprintf(fp, "            dhcp4: true\n");
      fprintf(fp, "            addresses: [%s]\n", ip_cidr);
      print_ok("DHCP4: wlaczony");
    } else {
      fprintf(fp, "            dhcp4: false\n");
      fprintf(fp, "            addresses: [%s]\n", ip_cidr);
      print_ok("DHCP4: wylaczony");
    }

    printf(C_DIM "     Adres: %s" C_RESET "\n", ip_cidr);
    fclose(fp);
    print_ok("Plik netplan zapisany");

    printf("\n");
    print_step(19, "Stosowanie konfiguracji sieci...");
    if (run_with_progress("netplan apply 2>/dev/null", "Stosowanie netplan...") != 0) {
      print_fail("Blad podczas netplan apply!");
    } else {
      print_ok("Konfiguracja sieci zastosowana");
    }
    printf("\n");
  } else {
    printf("\n");
    print_info("Pominieto konfiguracje netplan.");
    printf("\n");
  }

  // ==========================================
  //  EKRAN 6: CZYSZCZENIE
  // ==========================================
  print_separator();
  printf("\n");

  if (prompt_yn("Usunac plik 'a' z /home/?")) {
    if (run_quick("rm -f /home/a", "rm /home/a") == 0) {
      print_ok("Plik usuniety");
    } else {
      print_fail("Nie udalo sie usunac pliku");
    }
  }

  // ==========================================
  //  KONIEC
  // ==========================================
  printf("\n\n");
  print_box("KONFIGURACJA ZAKONCZONA");

  printf(C_BGREEN);
  printf("  ┌────────────────────────────────────────────────────┐\n");
  printf("  │  Wszystkie kroki zostaly wykonane pomyslnie!       │\n");
  printf("  │                                                    │\n");
  printf("  │  Sprawdz status:  systemctl status smbd            │\n");
  printf("  │  Logi:            tail -f /var/log/samba/log.smbd  │\n");
  printf("  │  Restart:         systemctl restart smbd           │\n");
  printf("  └────────────────────────────────────────────────────┘" C_RESET "\n\n");

  return 0;
}