pub const INDEX: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta property="og:title" content="{{ username }}' site'" />
  <meta property="og:image" content="/static/site-image.png" />

  <title>{{ username }}' site</title>
  <link rel="stylesheet" href="/static/css/index.css">
  <link rel="shortcut icon" type="image/jpg" href="/static/favicon.ico"/>
</head>

<body>
  <h1>Hi, I'm {{ username }}</h1>
  <p>
  I have a <a href="/blog">blog.</a><br>
  If you have questions or input for me please send me an E-Mail to {{ email }}
  </p>
</body>
</html>
"#;

pub const BLOG: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta property="og:title" content="{{ sitetitle }}" />
    <meta property="og:image" content="/static/site-image.png" />

    <title> {{ sitetitle }} </title>
    <link rel="stylesheet" href="/static/css/blog.css">
    <link rel="shortcut icon" type="image/jpg" href="/static/favicon.ico"/>
  </head>

  <body>
    <h1><a href="/blog" class="post-link" style="text-decoration:none;color:black;">{{ username }}' blog</a></h1>
    <ul>
      {% for post in posts %}
      <article>
        <a href="/blog/id/{{ post.id }}" class="post-link">[link]</a>
        <div class="post-content">
          <h2 class="post-title">{{ post.title }}</h2>
          <sub class="post-publish-date"> {{ post.publish_date | date(format="%Y-%m-%d at %H:%M") }}</sub>
          <p class="post-body">{{ post.body }}</p>
        </div>
      </article>
      {% endfor %}
    </ul>
  </body>
</html>
"#;

pub const SUBMIT: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta property="og:title" content="Submit post" />
    <meta property="og:image" content="/static/site-image.png" />

    <title>Submit post</title>
    <link rel="stylesheet" href="/static/css/blog.css">
    <link rel="shortcut icon" type="image/jpg" href="/static/favicon.ico"/>
  </head>
  <body>
    <div id="cookie-block" hidden>
      <p>Please set your token cookie first.</p>
      <input id="set-token" type="text" name="set-token">
      <button onclick="setTokenCookie()">Set Token Cookie</button>
    </div>
    <button onclick="clearTokenCookie()">Clear Token Cookie</button>

    <form id="submit-form" action="/api/blog/create" method=POST>
      <input class="token" type="text" name="token" hidden>
      <label for="title">Title</label>
      <textarea id="submit-title" type="text" name="title">{{ title }}</textarea>
      <br>
      <label for="submit-body">Content</label>
      <textarea id="submit-body" type="text" name="body">{{ body }}</textarea>
      <br>
      <button id="submit-button" type="submit">Submit</button>
    </form>
    <script src="/static/js/blog.js"></script>
  </body>
</html>
"#;

pub const EDIT: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta property="og:title" content="Edit posts..." />
    <meta property="og:image" content="/static/site-image.png" />

    <title>Edit posts...</title>
    <link rel="stylesheet" href="/static/css/blog.css">
    <link rel="shortcut icon" type="image/jpg" href="/static/favicon.ico"/>
  </head>
  <body>
    <h1><a href="/blog" class="post-link" style="text-decoration:none;color:black;">{{ username }}' blog</a></h1>
    <h2>Edit posts</h2>
    <ul style="list-style: none;">
      {% for post in posts %}
      <li><a href="/blog/edit/{{ post.id }}">{{ post.title }}</a></li>
      {% endfor %}
    </ul>
  </body>
</html>
"#;

pub const POST_EDIT_FORM: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta property="og:title" content="Edit '{{ title }}'"/>
    <meta property="og:image" content="/static/site-image.png" />

    <title>Edit '{{ title }}'</title>
    <link rel="stylesheet" href="/static/css/blog.css">
    <link rel="shortcut icon" type="image/jpg" href="/static/favicon.ico"/>
  </head>
  <body>
    <div id="cookie-block" hidden>
      <p>Please set your token cookie first.</p>
      <input id="set-token" type="text" name="set-token">
      <button onclick="setTokenCookie()">Set Token Cookie</button>
    </div>
    <button onclick="clearTokenCookie()">Clear Token Cookie</button>

    <form id="submit-form" action="/api/blog/posts/edit/{{ id }}" method=POST>
      <input class="token" type="text" name="token" hidden>
      <label for="title">Title</label>
      <textarea id="submit-title" type="text" name="title">{{ title }}</textarea>
      <br>
      <label for="submit-body">Content</label>
      <textarea id="submit-body" type="text" name="body">{{ body }}</textarea>
      <br>
      <button id="submit-button" type="submit">Edit post</button>
    </form>

    <form action="/api/blog/posts/hide/{{ id }}" method="POST">
      <input class="token" type="text" name="token" hidden>
      <button type="submit">Hide post</button>
    </form>
    <form action="/api/blog/posts/delete/{{ id }}" method="POST">
      <input class="token" type="text" name="token" hidden>
      <button type="submit">Delete post</button>
    </form>
    <script src="/static/js/blog.js"></script>
  </body>
</html>
"#;

