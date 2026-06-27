# Account Creation How-tos

## AD/Domain accounts

### Linux

#### Method A (happiest)

1. Look up user via username/email/SID with getent/id
   1. Get UID/GID info (error checking)
2. Use `mkhomedir_helper` to create user folder with UID/GID
   1. works without user's password
   2. only creates user folder with UID/GIDs correct. No entry in passwd, shadow, or group added
   3. 1st login should automatically sync up the account record with the created folder
3.  done?

### Win32

#### WET/USMT model
Probably has licensing issues with Windows ADK, but if already installed or informed to install could work.
1. Use custom rules for migration:
  ```xml
  <?xml version="1.0" encoding="UTF-8"?>
  <migration urlid="http://www.microsoft.com/migration/1.0/migxmlext/bareuser">

    <!--
      BareUserMigration.xml
      Goal: migrate the user ACCOUNT + user SETTINGS + APP SETTINGS,
            but NOT the user's data files (Documents, Pictures, Desktop, etc.).
      Use ALONGSIDE MigApp.xml. Do NOT use MigUser.xml or MigDocs.xml.

      How it works:
        - Core USMT manifests migrate the account, profile, and NTUSER.DAT (settings).
        - MigApp.xml migrates application settings.
        - The unconditionalExclude rules below strip the bulky DATA folders so the
          destination profile is essentially bare. AppData is preserved because that
          is where user/app settings live.
    -->

    <component type="Documents" context="System">
      <displayName>Exclude bulk user data folders (keep a bare profile)</displayName>
      <role role="Data">
        <rules>
          <unconditionalExclude>
            <objectSet>

              <!-- Known-folder data locations (per-user) -->
              <pattern type="File">%CSIDL_MYDOCUMENTS%\* [*]</pattern>
              <pattern type="File">%CSIDL_PERSONAL%\* [*]</pattern>
              <pattern type="File">%CSIDL_MYPICTURES%\* [*]</pattern>
              <pattern type="File">%CSIDL_MYMUSIC%\* [*]</pattern>
              <pattern type="File">%CSIDL_MYVIDEO%\* [*]</pattern>
              <pattern type="File">%CSIDL_DESKTOPDIRECTORY%\* [*]</pattern>
              <pattern type="File">%CSIDL_FAVORITES%\* [*]</pattern>

              <!-- Folders that have no CSIDL by default (reference under the profile root) -->
              <pattern type="File">%CSIDL_PROFILE%\Downloads\* [*]</pattern>
              <pattern type="File">%CSIDL_PROFILE%\Contacts\* [*]</pattern>
              <pattern type="File">%CSIDL_PROFILE%\Links\* [*]</pattern>
              <pattern type="File">%CSIDL_PROFILE%\Searches\* [*]</pattern>
              <pattern type="File">%CSIDL_PROFILE%\Saved Games\* [*]</pattern>
              <pattern type="File">%CSIDL_PROFILE%\OneDrive\* [*]</pattern>

            </objectSet>
          </unconditionalExclude>
        </rules>
      </role>
    </component>

  </migration>
  ```
2. Export migration info on old machine:
   1. `scanstate C:\store /i:migapp.xml /i:custom-mig.xml /ue:* /ui:DOMAIN\user /o /c`
3. Import info on new machine:
   1. `loadstate C:\store /i:migapp.xml /i:custom-mig.xml /mu:DOMAIN\user:DOMAIN\user /c`

#### Manual model
1. Create user folder
2. use `icacls` to setowner & substitute SID info
   1. `icacls <dst> /setowner "DOMAIN\user" /T /C`
   2. `icacls <dst> /substitute *<OLD_SID> *<NEW_SID> /T /C`
3. Pre-load user into registry:
   1. Add Hive/folder: `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\ProfileList\<SID>`
   2. Add keys:
| key | Type | Data |
|-----|------|------|
| ProfileImagePath | REG_EXPAND_SZ | C:/Users/<username> |
| Flags | REG_DWORD | 0 |
| State | REG_DWORD | 0 |  
| Flags | REG_DWORD | 0 |  


## Local accounts


## Edge Cases
- Add options for conflicting UIDs & usernames
  - Conflicting UID
    - Means different domain is used or was used beforehand (error)
  - Conflicting username
    - check SSSD config for fallback_homedir option. If set then not conflicting
    - Otherwise tell user (error) 
