function sendForm() {
    let username = (document.getElementById("username-input")! as HTMLInputElement) .value;
    let password = (document.getElementById("password-input")! as HTMLInputElement) .value;
    fetch("http://localhost:8001/api/accounts/login", {
        method: "POST",
        body: JSON.stringify({ username, password })
    })
        .then(response => {
            if (response.status == 200) {
                response.json().then(data => {
                    document.cookie = `user_token=${data.token}; path=/`;
                    window.location.href = "/todos";
                });
            } else {
                response.text().then(text => {
                    document.getElementById("error-message")!.innerHTML = text;
                })
            }
        })
}
