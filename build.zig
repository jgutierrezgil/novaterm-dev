const std = @import("std");
const builtin = @import("builtin");

// Función principal del build
pub fn build(b: *std.Build) void {
    // Parámetros de optimización por defecto
    const optimize = b.standardOptimizeOption(.{});

    // Target por defecto es la máquina host
    const target = b.standardTargetOptions(.{});

    // Definir el ejecutable principal
    const exe = b.addExecutable(.{
        .name = "novaterm",
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Integración de SDL2 usando zig-sdl
    // Nota: Asume que zig-sdl está en un subdirectorio 'deps/zig-sdl'
    const sdl_pkg = b.addModule("sdl2", .{
        .source_file = .{ .path = "deps/zig-sdl/src/sdl.zig" },
    });
    exe.addModule("sdl2", sdl_pkg);

    // Linkear con SDL2 de forma específica según el sistema
    switch (target.getOsTag()) {
        .linux => {
            exe.linkSystemLibrary("SDL2");
            exe.linkLibC();
        },
        .macos => {
            // En macOS, SDL2 se puede instalar via Homebrew
            exe.linkFramework("SDL2");
            exe.linkFramework("CoreVideo");
            exe.linkFramework("CoreAudio");
            exe.linkFramework("AudioToolbox");
        },
        else => {
            @panic("Sistema operativo no soportado");
        },
    }

    // Instalar el ejecutable en el sistema
    b.installArtifact(exe);

    // Crear un step para ejecutar el programa
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    // Agregar el comando 'run' como un custom step
    const run_step = b.step("run", "Ejecutar novaterm");
    run_step.dependOn(&run_cmd.step);

    // Configuración para cross-compilation
    const linux_x86_64 = b.resolveTargetQuery(.{
        .cpu_arch = .x86_64,
        .os_tag = .linux,
    });

    const macos_arm64 = b.resolveTargetQuery(.{
        .cpu_arch = .aarch64,
        .os_tag = .macos,
    });

    // Crear ejecutables para cada target
    const exe_linux = createExeForTarget(b, optimize, linux_x86_64, "novaterm-linux");
    const exe_macos = createExeForTarget(b, optimize, macos_arm64, "novaterm-macos");

    // Agregar steps para compilación cruzada
    const build_all = b.step("build-all", "Compilar para todas las plataformas soportadas");
    build_all.dependOn(&exe_linux.step);
    build_all.dependOn(&exe_macos.step);
}

// Función auxiliar para crear ejecutables para diferentes targets
fn createExeForTarget(
    b: *std.Build,
    optimize: std.builtin.OptimizeMode,
    target: std.Build.ResolvedTarget,
    name: []const u8,
) *std.Build.CompileStep {
    const exe = b.addExecutable(.{
        .name = name,
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Configuración específica según el target
    switch (target.getOsTag()) {
        .linux => {
            exe.linkSystemLibrary("SDL2");
            exe.linkLibC();
        },
        .macos => {
            exe.linkFramework("SDL2");
            exe.linkFramework("CoreVideo");
            exe.linkFramework("CoreAudio");
            exe.linkFramework("AudioToolbox");
        },
        else => unreachable,
    }

    b.installArtifact(exe);
    return exe;
}
