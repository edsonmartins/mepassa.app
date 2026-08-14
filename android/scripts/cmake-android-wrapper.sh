#!/bin/bash
# Wrapper do cmake para cross-compilar a lib Rust core para Android.
#
# O crate `cmake` (usado pelo audiopus_sys/libopus) não injeta explicitamente
# ANDROID_ABI/ANDROID_PLATFORM, e o CMake 4+ rejeita o `cmake_minimum_required`
# antigo do projeto opus. Este wrapper resolve ambos:
#   - injeta -DANDROID_ABI=<arch> -DANDROID_PLATFORM=android-24
#   - injeta -DCMAKE_POLICY_VERSION_MINIMUM=3.5
# APENAS na fase de configuração (não injeta em `cmake --build`, onde -D é
# inválido). A arquitetura é inferida de CARGO_BUILD_TARGET/TARGET.
real_cmake=$(command -v cmake)
args=("$@")

# Detecta se é a fase de build (--build/--install/--preset) — não injeta -D.
is_build="no"
for a in "${args[@]}"; do
  case "$a" in
    --build|--install|--preset) is_build="yes" ;;
  esac
done

abi=""
platform="android-24"
case "${CARGO_BUILD_TARGET:-$TARGET}" in
  *aarch64*) abi="arm64-v8a" ;;
  *armv7*)   abi="armeabi-v7a" ;;
  *x86_64*)  abi="x86_64" ;;
esac

if [ "$is_build" = "yes" ] || [ -z "$abi" ]; then
  exec "$real_cmake" "${args[@]}"
fi

newargs=()
has_abi="no"
has_pol="no"
for a in "${args[@]}"; do
  case "$a" in
    -DANDROID_ABI=*) has_abi="yes" ;;
    -DCMAKE_POLICY_VERSION_MINIMUM=*) has_pol="yes" ;;
  esac
  newargs+=("$a")
done
if [ "$has_abi" = "no" ]; then
  newargs+=("-DANDROID_ABI=$abi" "-DANDROID_PLATFORM=$platform")
fi
if [ "$has_pol" = "no" ]; then
  newargs+=("-DCMAKE_POLICY_VERSION_MINIMUM=3.5")
fi
exec "$real_cmake" "${newargs[@]}"