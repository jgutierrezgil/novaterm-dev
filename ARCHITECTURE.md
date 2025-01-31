# Arquitectura de NovaTerm

```mermaid
flowchart TD
    A[Binario Principal] --> B[Core Terminal]
    A --> C[GUI]
    B --> D[Parser ANSI]
    B --> E[Gestión de Procesos]
    C --> F[Renderizador GPU]
    C --> G[Sistema de Ventanas]
```

## Módulos Principales

### `core`
- **Responsabilidad**: Lógica pura de terminal
- **Dependencias**: 
  - `nom`: Parsing ANSI
  - `tokio`: Ejecución async de procesos

### `gui`
- **Stack Técnico**:
  - `iced`: Renderizado declarativo
  - `glow`: Abstracción OpenGL/Vulkan
  - `cosmic-text`: Layout de texto avanzado