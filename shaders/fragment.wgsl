fn map(p: vec3<f32>) -> f32 {
    let sphere = length(p) - 1.0;
    let floor = abs(p.y - 1.2);
    return min(sphere, floor);
}

// Aproximación de la normal usando diferencias finitas
fn calcNormal(p: vec3<f32>) -> vec3<f32> {
    let eps: f32 = 0.001;
    let nx = map(p + vec3f(eps, 0.0, 0.0)) - map(p - vec3f(eps, 0.0, 0.0));
    let ny = map(p + vec3f(0.0, eps, 0.0)) - map(p - vec3f(0.0, eps, 0.0));
    let nz = map(p + vec3f(0.0, 0.0, eps)) - map(p - vec3f(0.0, 0.0, eps));
    return normalize(vec3f(nx, ny, nz));
}

// Cálculo del Ambient Occlusion
fn calcAO(p: vec3<f32>, n: vec3<f32>) -> f32 {
    var occlusion: f32 = 0.0;
    var scale: f32 = 1.0;
    // Se realizan 5 muestreos a lo largo de la normal
    for (var i = 1; i <= 5; i++) {
        let h = f32(i) * 0.1;
        let d = map(p + n * h);
        occlusion += (h - d) * scale;
        scale *= 0.5;
    }
    return clamp(1.0 - occlusion, 0.0, 1.0);
}

@fragment
fn main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(800.0, 600.0);
    let uv = (frag_coord.xy * 2.0 - resolution) / resolution.y;
    
    // Rayo primario
    var ro = vec3f(0.0, 0.0, -3.0);
    var rd = normalize(vec3f(uv, 1.0));
    
    // Ray marching para hallar la intersección
    var t = 0.0;
    var i = 0;
    var p = vec3f(0.0);
    
    for (; i < 80; i++) {
        p = ro + rd * t;
        let d = map(p);
        t += d;
        if (d < 0.001 || t > 100.0) { break; }
    }
    
    // Punto de intersección
    let hitPos = ro + rd * t;
    
    // Cálculo de la dirección de la luz directa (por ejemplo, simulando al sol)
    let lightDir = normalize(vec3f(100.0, -100.0, 0.0));
    
    // Cálculo de sombras suaves (direct lighting)
    var shadow: f32 = 1.0;
    var tShadow = 0.02;
    for (i = 0; i < 80; i++) {
        let pShadow = hitPos + lightDir * tShadow;
        let h = map(pShadow);
        shadow = min(shadow, 16.0 * h / tShadow);
        tShadow += h;
        if (h < 0.001 || tShadow > 100.0) { break; }
    }
    
    // Calcular la normal en el punto de intersección y el AO
    let n = calcNormal(hitPos);
    let ao = calcAO(hitPos, n);
    
    // Combinar luz directa e indirecta
    let directLight = shadow * max(dot(n, lightDir), 0.0);
    let ambientLight = 0.2 * ao; // Factor de luz indirecta (ajusta 0.2 según lo deseado)
    let lighting = directLight + ambientLight;
    
    // Color final (puedes combinar con otros términos, por ejemplo, basados en t)
    let col = vec3f(lighting * 1.0 + t * 0.0);
    return vec4<f32>(col, 1.0);
}

