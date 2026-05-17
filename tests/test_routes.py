import uuid

import pytest
import requests


BASE_URL = "http://localhost:8080"


# =========================================================
# Helpers
# =========================================================

def assert_no_internal_error(text: str):
    assert "Internal Server Error" not in text


def random_user():
    suffix = uuid.uuid4().hex[:8]

    return {
        "username": f"user_{suffix}",
        "password": "password123",
    }


def signup(session: requests.Session, username: str, password: str):
    return session.post(
        f"{BASE_URL}/signup",
        data={
            "username": username,
            "password": password,
        },
        allow_redirects=False,
    )


def login(session: requests.Session, username: str, password: str):
    return session.post(
        f"{BASE_URL}/login",
        data={
            "username": username,
            "password": password,
        },
        allow_redirects=False,
    )


def auth_cookies(user):
    return {
        "username": user["username"],
        "password": user["password"],
    }


# =========================================================
# GET /
# =========================================================

def test_get_index_without_auth():
    response = requests.get(f"{BASE_URL}/")

    assert response.status_code == 200
    assert_no_internal_error(response.text)

    assert "Invalid username or password" not in response.text


def test_get_index_with_auth_redirects_dashboard():
    user = random_user()

    session = requests.Session()

    signup(session, user["username"], user["password"])

    response = requests.get(
        f"{BASE_URL}/",
        cookies=auth_cookies(user),
        allow_redirects=False,
    )

    assert response.status_code in (301, 302, 303, 307, 308)
    assert response.headers["Location"] == "/dashboard"


# =========================================================
# POST /login
# =========================================================

def test_login_success():
    user = random_user()

    session = requests.Session()

    signup(session, user["username"], user["password"])

    response = login(
        session,
        user["username"],
        user["password"],
    )

    assert response.status_code in (301, 302, 303, 307, 308)
    assert response.headers["Location"] == "/dashboard"

    assert session.cookies.get("username") == user["username"]
    assert session.cookies.get("password") == user["password"]


def test_login_invalid_credentials():
    session = requests.Session()

    response = login(
        session,
        "invalid_user",
        "invalid_password",
    )

    assert response.status_code in (301, 302, 303, 307, 308)
    assert response.headers["Location"] == "/"

    follow = session.get(f"{BASE_URL}/")

    assert follow.status_code == 200
    assert_no_internal_error(follow.text)

    assert "Invalid username or password" in follow.text


# =========================================================
# GET /signup
# =========================================================

def test_get_signup_without_auth():
    response = requests.get(f"{BASE_URL}/signup")

    assert response.status_code == 200
    assert_no_internal_error(response.text)

    assert "Username taken" not in response.text


def test_get_signup_with_auth_redirects_dashboard():
    user = random_user()

    session = requests.Session()

    signup(session, user["username"], user["password"])

    response = requests.get(
        f"{BASE_URL}/signup",
        cookies=auth_cookies(user),
        allow_redirects=False,
    )

    assert response.status_code in (301, 302, 303, 307, 308)
    assert response.headers["Location"] == "/dashboard"


# =========================================================
# POST /signup
# =========================================================

def test_signup_success():
    user = random_user()

    session = requests.Session()

    response = signup(
        session,
        user["username"],
        user["password"],
    )

    assert response.status_code in (301, 302, 303, 307, 308)
    assert response.headers["Location"] == "/dashboard"

    assert session.cookies.get("username") == user["username"]
    assert session.cookies.get("password") == user["password"]


def test_signup_existing_user():
    user = random_user()

    session = requests.Session()

    signup(session, user["username"], user["password"])

    second_session = requests.Session()

    response = signup(
        second_session,
        user["username"],
        user["password"],
    )

    assert response.status_code in (301, 302, 303, 307, 308)
    assert response.headers["Location"] == "/signup"

    follow = second_session.get(f"{BASE_URL}/signup")

    assert follow.status_code == 200
    assert_no_internal_error(follow.text)

    assert "Username taken" in follow.text


def test_signup_invalid_username():
    session = requests.Session()

    response = signup(
        session,
        "../evil",
        "password123",
    )

    assert response.status_code in (301, 302, 303, 307, 308)
    assert response.headers["Location"] == "/signup"

    follow = session.get(f"{BASE_URL}/signup")

    assert "Username should not contain" in follow.text
    assert_no_internal_error(follow.text)


# =========================================================
# POST /logout
# =========================================================

def test_logout_removes_credentials():
    user = random_user()

    session = requests.Session()

    signup(session, user["username"], user["password"])

    response = session.post(
        f"{BASE_URL}/logout",
        allow_redirects=False,
    )

    assert response.status_code in (301, 302, 303, 307, 308)
    assert response.headers["Location"] == "/"

    dashboard = session.get(
        f"{BASE_URL}/dashboard",
        allow_redirects=False,
    )

    assert dashboard.status_code in (301, 302, 303, 307, 308)
    assert dashboard.headers["Location"] == "/"


# =========================================================
# GET /dashboard
# =========================================================

def test_dashboard_requires_auth():
    response = requests.get(
        f"{BASE_URL}/dashboard",
        allow_redirects=False,
    )

    assert response.status_code in (301, 302, 303, 307, 308)
    assert response.headers["Location"] == "/"


def test_dashboard_empty():
    user = random_user()

    session = requests.Session()

    signup(session, user["username"], user["password"])

    response = session.get(f"{BASE_URL}/dashboard")

    assert response.status_code == 200
    assert_no_internal_error(response.text)

    assert "No files uploaded yet" in response.text
    assert user["username"] in response.text
    assert "Upload" in response.text
    assert "Exit" in response.text


# =========================================================
# POST /upload-file
# =========================================================

def test_upload_file_success():
    user = random_user()

    session = requests.Session()

    signup(session, user["username"], user["password"])

    response = session.post(
        f"{BASE_URL}/upload-file",
        files={
            "file": ("hello.txt", b"hello world"),
        },
        allow_redirects=False,
    )

    assert response.status_code in (301, 302, 303, 307, 308)
    assert response.headers["Location"] == "/dashboard"

    dashboard = session.get(f"{BASE_URL}/dashboard")

    assert "hello.txt" in dashboard.text
    assert "Download" in dashboard.text
    assert "Delete" in dashboard.text

    assert_no_internal_error(dashboard.text)


def test_upload_requires_auth():
    response = requests.post(
        f"{BASE_URL}/upload-file",
        files={
            "file": ("hello.txt", b"hello"),
        },
        allow_redirects=False,
    )

    assert response.status_code in (301, 302, 303, 307, 308)
    assert response.headers["Location"] == "/"


# =========================================================
# POST /delete-file
# =========================================================

def test_delete_file_success():
    user = random_user()

    session = requests.Session()

    signup(session, user["username"], user["password"])

    session.post(
        f"{BASE_URL}/upload-file",
        files={
            "file": ("delete_me.txt", b"data"),
        },
    )

    response = session.post(
        f"{BASE_URL}/delete-file",
        data={
            "filename": "delete_me.txt",
        },
        allow_redirects=False,
    )

    assert response.status_code in (301, 302, 303, 307, 308)
    assert response.headers["Location"] == "/dashboard"

    dashboard = session.get(f"{BASE_URL}/dashboard")

    assert "delete_me.txt" not in dashboard.text

    assert_no_internal_error(dashboard.text)


def test_delete_requires_auth():
    response = requests.post(
        f"{BASE_URL}/delete-file",
        data={
            "filename": "hello.txt",
        },
        allow_redirects=False,
    )

    assert response.status_code in (301, 302, 303, 307, 308)
    assert response.headers["Location"] == "/"


# =========================================================
# GET /file
# =========================================================

def test_get_file_success():
    user = random_user()

    session = requests.Session()

    signup(session, user["username"], user["password"])

    file_data = b"hello world"

    session.post(
        f"{BASE_URL}/upload-file",
        files={
            "file": ("hello.txt", file_data),
        },
    )

    response = session.get(
        f"{BASE_URL}/file",
        params={
            "filename": "hello.txt",
        },
    )

    assert response.status_code == 200

    assert (
        response.headers["Content-Type"]
        == "application/octet-stream"
    )

    assert (
        'attachment; filename = "hello.txt"'
        in response.headers["Content-Disposition"]
    )

    assert response.content == file_data


def test_get_missing_file_returns_empty_body():
    user = random_user()

    session = requests.Session()

    signup(session, user["username"], user["password"])

    response = session.get(
        f"{BASE_URL}/file",
        params={
            "filename": "missing.txt",
        },
    )

    assert response.status_code == 200
    assert response.content == b""


def test_get_file_requires_auth():
    response = requests.get(
        f"{BASE_URL}/file",
        params={
            "filename": "hello.txt",
        },
        allow_redirects=False,
    )

    assert response.status_code in (301, 302, 303, 307, 308)
    assert response.headers["Location"] == "/"
