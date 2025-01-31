# Estándares de Código

1. **Sistema de Tipos**:
   - Evitar `unwrap()` → Usar `?` o manejo explícito
   - Tipos nuevos (`struct`) en lugar de primitivos para dominios complejos

2. **Concurrencia**:
   - Preferir `tokio::spawn` sobre `std::thread`
   - Canales (`mpsc`) para comunicación entre módulos

3. **Documentación**:
   - Ejemplos de uso en `/// Examples`
   - Tests de documentación con `cargo test --doc`