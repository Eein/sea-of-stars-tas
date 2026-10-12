#include <windows.h>
#include <xinput.h>
#include <stdint.h>
#include <string.h>

// Written by crates/joystick/src/crossover.rs next to this DLL.
// The layout is exactly 16 bytes; all multi-byte fields are little-endian.
typedef struct {
    char magic[4];
    uint16_t buttons;
    uint8_t left_trigger;
    uint8_t right_trigger;
    int16_t left_x;
    int16_t left_y;
    int16_t right_x;
    int16_t right_y;
} TasStateFile;

typedef char TasStateFileMustBe16Bytes[sizeof(TasStateFile) == 16 ? 1 : -1];

static HMODULE self_module;
static SRWLOCK state_lock = SRWLOCK_INIT;
static XINPUT_GAMEPAD previous_gamepad;
static DWORD packet_number;

BOOL WINAPI DllMain(HINSTANCE instance, DWORD reason, LPVOID reserved)
{
    (void)reserved;
    if (reason == DLL_PROCESS_ATTACH) self_module = instance;
    return TRUE;
}

static void read_gamepad(XINPUT_GAMEPAD *gamepad)
{
    char path[MAX_PATH];
    char *separator;
    HANDLE file;
    DWORD bytes_read = 0;
    TasStateFile state;

    ZeroMemory(gamepad, sizeof(*gamepad));
    DWORD path_length = GetModuleFileNameA(self_module, path, sizeof(path));
    if (!path_length || path_length >= sizeof(path)) return;
    separator = strrchr(path, '\\');
    if (!separator || sizeof(path) - (size_t)(separator + 1 - path) < sizeof("tas-xinput-state.bin")) return;
    strcpy(separator + 1, "tas-xinput-state.bin");

    file = CreateFileA(path, GENERIC_READ,
                       FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                       NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
    if (file == INVALID_HANDLE_VALUE) return;
    if (ReadFile(file, &state, sizeof(state), &bytes_read, NULL) &&
        bytes_read == sizeof(state) && memcmp(state.magic, "TAS1", 4) == 0) {
        gamepad->wButtons = state.buttons;
        gamepad->bLeftTrigger = state.left_trigger;
        gamepad->bRightTrigger = state.right_trigger;
        gamepad->sThumbLX = state.left_x;
        gamepad->sThumbLY = state.left_y;
        gamepad->sThumbRX = state.right_x;
        gamepad->sThumbRY = state.right_y;
    }
    CloseHandle(file);
}

DWORD WINAPI XInputGetState(DWORD user_index, XINPUT_STATE *state)
{
    XINPUT_GAMEPAD gamepad;
    if (user_index != 0) return ERROR_DEVICE_NOT_CONNECTED;
    if (!state) return ERROR_BAD_ARGUMENTS;

    read_gamepad(&gamepad);
    AcquireSRWLockExclusive(&state_lock);
    if (memcmp(&gamepad, &previous_gamepad, sizeof(gamepad)) != 0) {
        previous_gamepad = gamepad;
        ++packet_number;
    }
    state->dwPacketNumber = packet_number;
    state->Gamepad = gamepad;
    ReleaseSRWLockExclusive(&state_lock);
    return ERROR_SUCCESS;
}

DWORD WINAPI XInputGetStateEx(DWORD user_index, XINPUT_STATE *state)
{
    return XInputGetState(user_index, state);
}

DWORD WINAPI XInputSetState(DWORD user_index, XINPUT_VIBRATION *vibration)
{
    if (user_index != 0) return ERROR_DEVICE_NOT_CONNECTED;
    if (!vibration) return ERROR_BAD_ARGUMENTS;
    return ERROR_SUCCESS; // This virtual controller has no rumble output.
}

DWORD WINAPI XInputGetCapabilities(DWORD user_index, DWORD flags, XINPUT_CAPABILITIES *caps)
{
    (void)flags;
    if (user_index != 0) return ERROR_DEVICE_NOT_CONNECTED;
    if (!caps) return ERROR_BAD_ARGUMENTS;
    ZeroMemory(caps, sizeof(*caps));
    caps->Type = XINPUT_DEVTYPE_GAMEPAD;
    caps->SubType = XINPUT_DEVSUBTYPE_GAMEPAD;
    caps->Gamepad.wButtons = 0xffff;
    caps->Gamepad.bLeftTrigger = 255;
    caps->Gamepad.bRightTrigger = 255;
    caps->Gamepad.sThumbLX = 32767;
    caps->Gamepad.sThumbLY = 32767;
    caps->Gamepad.sThumbRX = 32767;
    caps->Gamepad.sThumbRY = 32767;
    return ERROR_SUCCESS;
}

void WINAPI XInputEnable(BOOL enable)
{
    (void)enable;
}

DWORD WINAPI XInputGetDSoundAudioDeviceGuids(DWORD user_index, GUID *render, GUID *capture)
{
    if (user_index != 0) return ERROR_DEVICE_NOT_CONNECTED;
    if (render) ZeroMemory(render, sizeof(*render));
    if (capture) ZeroMemory(capture, sizeof(*capture));
    return ERROR_SUCCESS;
}

DWORD WINAPI XInputGetBatteryInformation(DWORD user_index, BYTE device_type,
                                         XINPUT_BATTERY_INFORMATION *battery)
{
    (void)device_type;
    if (user_index != 0) return ERROR_DEVICE_NOT_CONNECTED;
    if (!battery) return ERROR_BAD_ARGUMENTS;
    battery->BatteryType = BATTERY_TYPE_WIRED;
    battery->BatteryLevel = BATTERY_LEVEL_FULL;
    return ERROR_SUCCESS;
}

DWORD WINAPI XInputGetKeystroke(DWORD user_index, DWORD reserved, XINPUT_KEYSTROKE *keystroke)
{
    (void)reserved;
    if (user_index != 0 && user_index != XUSER_INDEX_ANY) return ERROR_DEVICE_NOT_CONNECTED;
    if (!keystroke) return ERROR_BAD_ARGUMENTS;
    return ERROR_EMPTY;
}
