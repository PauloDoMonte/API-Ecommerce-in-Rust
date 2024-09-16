// routes/home.rs


use rocket::get;
use rocket::response::content::RawHtml;

#[get("/")]
pub fn home() -> RawHtml<String> {
    let routes = vec![
        ("(home)", "GET", "/", "Página inicial do aplicativo."),
        ("(get_users)", "GET", "/user/", "Retorna a lista de usuários."),
        ("(login)", "GET", "/login/", "Página de login."),
        ("(product)", "GET", "/produto/", "Página do produto."),
        ("(register_post)", "POST", "/register/", "Registra um novo usuário."),
        ("(register_get)", "GET", "/register/", "Página de registro."),
    ];

    let mut html = String::from(r#"
        <html>
        <head>
            <title>Página Inicial</title>
            <link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css">
        </head>
        <body>
            <div class="container">
                <h1 class="mt-5">Bem-vindo à página inicial!</h1>
                <h2 class="mt-4">Lista de Rotas</h2>
                <table class="table table-striped">
                    <thead>
                        <tr>
                            <th>Rota</th>
                            <th>Método</th>
                            <th>URL</th>
                            <th>Descrição</th>
                        </tr>
                    </thead>
                    <tbody>
    "#);

    for (name, method, path, description) in routes {
        html.push_str(&format!(
            "<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
            </tr>",
            name, method, path, description
        ));
    }

    html.push_str(r#"
                    </tbody>
                </table>
            </div>
            <script src="https://code.jquery.com/jquery-3.5.1.slim.min.js"></script>
            <script src="https://cdn.jsdelivr.net/npm/@popperjs/core@2.9.2/dist/umd/popper.min.js"></script>
            <script src="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/js/bootstrap.min.js"></script>
        </body>
        </html>
    "#);
    RawHtml(html)
}
