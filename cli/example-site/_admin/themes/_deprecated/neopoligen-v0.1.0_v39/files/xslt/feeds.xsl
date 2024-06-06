<?xml version="1.0" encoding="UTF-8"?>

<xsl:stylesheet version="1.0"
    xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
    xmlns:a="http://www.w3.org/2005/Atom">
  <xsl:output method="html" encoding="UTF-8"/>
  <xsl:template match="a:feed">
    <html>
      <head>
        <title>
          <xsl:value-of select="a:title"/>
        </title>
<link rel="stylesheet" href="/theme/styles/variables.css" />
<link rel="stylesheet" href="/theme/styles/theme.css" />
<link rel="stylesheet" href="/theme/styles/syntax-highlighting/code-base16-ocean-dark.css" />
<link rel="stylesheet" href="/theme/styles/lite-yt-embed.css" />
      </head>

<body class="one-column">
    <header>
            <a href="/">home</a>
                </header>
    <main class="flow">
        <h1>alanwsmith.com <br />rss feed</h1>
                
                    <p>This is the RSS Feed for alanwsmith.com. 
                        If you're browser hasn't already taken care of it, 
                        you can copy and paste that address/url into your
                        news reader to subscribe to it. If you don't already
                        have a news reader, I like <a href="https://netnewswire.com/">NetNewsWire</a>
                        for mac/iphone/ipad. It's free. (I'm afraid I don't know 
                        enough about windows news readers to recommend one at
                        the moment)
                        </p>
                    <p>Please note: I'm in the process of moving my site and 
                    lots of stuff is broken. Things will straighten up in the
                        next couple of weeks</p>

                    <p>The current items in the feed are:</p>
                    <ul>
<xsl:apply-templates select="a:entry" />
                        </ul>

                    </main>
      </body>
    </html>
  </xsl:template> 


  <xsl:template match="a:entry">
    <li>
      <xsl:element name="a">
          <xsl:attribute name="href">
                    <xsl:value-of select="a:link[@rel='alternate']/@href"/>
          </xsl:attribute>
          <xsl:value-of select="a:title"/>
      </xsl:element>
    </li>
  </xsl:template> 

</xsl:stylesheet>

