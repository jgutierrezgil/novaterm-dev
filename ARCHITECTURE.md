# Arquitectura de NovaTerm

```mermaid
graph TD
    A[Core] --> B[Parser ANSI]
    A --> C[Gestión de Procesos]
    D[GUI] --> E[Render SDL2]
    D --> F[Input Handling]
    G[Plugins] --> H[WASM Runtime]
```

## Principios
- **Minimalismo**: 0 dependencias externas excepto SDL2
- **Seguridad**: Sandboxing via Landlock (Linux)
- **Extensibilidad**: API semántica v1 desde el inicio