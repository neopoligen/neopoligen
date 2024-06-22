use crate::neo_error::*;
use crate::section::*;
use crate::site_config::ConfigSections;
use nom::character::complete::multispace0;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::error::GenericErrorTree;
use nom_supreme::final_parser::final_parser;
use nom_supreme::final_parser::Location;
use nom_supreme::final_parser::RecreateContext;
use nom_supreme::parser_ext::ParserExt;

pub fn parse_ast(source: &str, sections: ConfigSections) -> Result<Vec<Section>, NeoError> {
    match final_parser(|src| do_parse(src, &sections))(source) {
        Ok(data) => Ok(data),
        Err(e) => Err(get_error(source, &e).into()),
    }
}

fn do_parse<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
) -> IResult<&'a str, Vec<Section>, ErrorTree<&'a str>> {
    let (source, _) = multispace0(source)?;
    let (source, result) = many1(|src| start_or_full_section(src, &sections))
        .context("page")
        .parse(source)?;
    Ok((source, result))
}

fn get_error(content: &str, tree: &ErrorTree<&str>) -> NeoError {
    match tree {
        GenericErrorTree::Base { location, kind } => {
            let details = Location::recreate_context(content, location);

            NeoError {
                kind: NeoErrorKind::ParserError {
                    source_path: None,
                    line: details.line,
                    column: details.column,
                    source: content.to_string(),
                    remainder: location.to_string(),
                    message: kind.to_string(),
                },
            }

            //
        }
        GenericErrorTree::Stack { contexts, .. } => {
            let context = contexts[0];
            let details = Location::recreate_context(content, context.0);

            NeoError {
                kind: NeoErrorKind::ParserError {
                    source_path: None,
                    line: details.line,
                    column: details.column,
                    source: content.to_string(),
                    remainder: context.0.to_string(),
                    message: context.1.to_string(),
                },
            }

            //
        }
        GenericErrorTree::Alt(items) => get_error(content, &items[0]),
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::site_config::SiteConfig;

    #[test]
    fn problem_finder_alfa() {
        let source = r#"-- title

Finally Got ffmpeg-concat To Install On An EC2 Machine - Stream Notes for November 10 2020 LiveCoding

-- youtube
-- 5MAnLKTXQIY

-- p

### Notes

-- p

Finally got ffmpeg-concat to install on an EC2 instance. 
One of those things where I spent hours and hours to find 
a few lines of code. And, not really code. Just install commands.

-- p

### Links From The Stream

-- note

These links need to be redone after migrating the them
into Neopoligen


-- pre/


- <<link|"SyntaxError: Unexpected token" on Debian 9 · Issue #3 · transitive-bullshit/ffmpeg-concat|https://github.com/transitive-bullshit/ffmpeg-concat/issues/3>>
- <<link|/306_ISS-TechDevelopment/306_ISS-TechDevelopment~orig.mp4|https://images-assets.nasa.gov/video/306_ISS-TechDevelopment/306_ISS-TechDevelopment~orig.mp4>>
- <<link|/VAFB-20180505-VP-...|https://images-assets.nasa.gov/video/VAFB-20180505-VP-CDC01_0001-InSight_Launch_Commentary-3189704/VAFB-20180505-VP-CDC01_0001-InSight_Launch_Commentary-3189704~orig.mp4>>
- <<link|8.4.5. Adding, Enabling, and Disabling a Yum Repository Red Hat Enterprise...|https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/sec-managing_yum_repositories>>
- <<link|92% chance Joe Biden will win in 2020 | Latest odds|https://odds.watch/joe-biden-2020>>
- <<link|A Beginner’s Guide to npm, the Node Package Manager - SitePoint|https://www.sitepoint.com/npm-guide/>>
- <<link|Amazon EC2 C5 Instances — Amazon Web Services (AWS)|https://aws.amazon.com/ec2/instance-types/c5/>>
- <<link|Amazon EC2 Instance Comparison|https://www.ec2instances.info/?reserved_term=yrTerm1Standard.partialUpfront>>
- <<link|Amazon EC2 Instance Types - Amazon Web Services|https://aws.amazon.com/ec2/instance-types/>>
- <<link|Amazon Linux AMI 2017.03 Packages|https://aws.amazon.com/amazon-linux-ami/2017.03-packages/>>
- <<link|Amazon Web Services (AWS) - Cloud Computing Services|https://aws.amazon.com/>>
- <<link|Ansible - Short Introduction | Vagrant by HashiCorp|https://www.vagrantup.com/docs/provisioning/ansible_intro>>
- <<link|AntTweakBar GUI library to tweak parameters of your OpenGL and DirectX...|http://anttweakbar.sourceforge.net/doc/>>
- <<link|App-cpanminus-1.7044 - get, unpack, build and install modules from CPAN - metacpan.org|https://metacpan.org/release/App-cpanminus>>
- <<link|Arch Linux - ArchWiki|https://wiki.archlinux.org/index.php/Arch_Linux>>
- <<link|asciinema - Record and share your terminal sessions, the right way|https://asciinema.org/>>
- <<link|Ask Ubuntu: Failed to create OpenGL context. Your graphics card must support...|https://askubuntu.com/questions/751324/failed-to-create-opengl-context-your-graphics-card-must-support-at-least-opengl>>
- <<link|AWS Developer Forums: SUSE instance and username ...|https://forums.aws.amazon.com/thread.jspa?threadID=171911>>
- <<link|AWS EC2 Instance Comparison: C4 vs C5|https://www.learnaws.org/2017/11/17/comparing-ec2-c4-c5/>>
- <<link|AWS ECU vs vCPU: Everything You Need to Know About EC2|https://www.virtana.com/blog/aws-ecu-vcpu/>>
- <<link|binding.gyp not found (cwd: c:\Users\xxxx\xxxx\) while trying to load...|https://github.com/nodejs/node-gyp/issues/702>>
- <<link|bindings - npm|https://www.npmjs.com/package/bindings>>
- <<link|Cannot create OpenGL context on server · Issue #189 · moderngl/moderngl|https://github.com/moderngl/moderngl/issues/189>>
- <<link|command line - crossfade between 2 videos using ffmpeg - Super User|https://superuser.com/questions/778762/crossfade-between-2-videos-using-ffmpeg/1559967>>
- <<link|command line - How to merge multiple (more than two) videos on Ubuntu? - Ask Ubuntu|https://askubuntu.com/questions/637074/how-to-merge-multiple-more-than-two-videos-on-ubuntu>>
- <<link|command line - Package x11 was not found in search path? - Ask Ubuntu|https://askubuntu.com/questions/536334/package-x11-was-not-found-in-search-path>>
- <<link|cpan - easily interact with CPAN from the command line - metacpan.org|https://metacpan.org/pod/distribution/CPAN/scripts/cpan>>
- <<link|CPAN - query, download and build perl modules from CPAN sites - Perldoc Browser|https://perldoc.perl.org/CPAN>>
- <<link|CPAN – DreamHost Knowledge Base|https://help.dreamhost.com/hc/en-us/articles/217716877-CPAN>>
- <<link|debian - How to reinstall a package using 'apt-get'? - Super User|https://superuser.com/questions/102449/how-to-reinstall-a-package-using-apt-get>>
- <<link|Define 7 — Fractal Design|https://www.fractal-design.com/products/cases/define/define-7/black-white/>>
- <<link|distributions/README.md at master · nodesource/distributions|https://github.com/nodesource/distributions/blob/master/README.md#debinstall>>
- <<link|does not launch on Ubuntu 18.04 (OpenGL error) :: Godhood Bugs and Technical Issues|https://steamcommunity.com/app/917150/discussions/3/1639790664946863546/>>
- <<link|Download | Node.js|https://nodejs.org/en/download/>>
- <<link|Downloading and installing Node.js and npm | npm Docs|https://docs.npmjs.com/downloading-and-installing-node-js-and-npm>>
- <<link|EC2 On-Demand Instance Pricing – Amazon Web Services|https://aws.amazon.com/ec2/pricing/on-demand/>>
- <<link|Error to compile x11 dependency · Issue #15 · ostrosco/device_query|https://github.com/ostrosco/device_query/issues/15>>
- <<link|error while running Dockerfile · Issue #45 · facebookresearch/House3D|https://github.com/facebookresearch/House3D/issues/45>>
- <<link|Error: Could not locate the bindings file · Issue #1511 · nodejs/node-gyp|https://github.com/nodejs/node-gyp/issues/1511>>
- <<link|Error: Could not locate the bindings file. · Issue #253 · libxmljs/libxmljs|https://github.com/libxmljs/libxmljs/issues/253>>
- <<link|Error: ENOTEMPTY: directory not empty · Issue #59 · johnagan/clean-webpack-plugin|https://github.com/johnagan/clean-webpack-plugin/issues/59>>
- <<link|ExifTool by Phil Harvey|https://exiftool.org/index.html>>
- <<link|Failed to create OpenGL context on Linux - fatal · Issue #427 · mltframework/shotcut|https://github.com/mltframework/shotcut/issues/427>>
- <<link|fedora - libX11.so.6 Not found - Unix & Linux Stack Exchange|https://unix.stackexchange.com/questions/1162/libx11-so-6-not-found>>
- <<link|FFmpeg on ubuntu|https://gist.github.com/isaactzab/7a259747dbc9c351777e>>
- <<link|ffmpeg-concat - npm|https://www.npmjs.com/package/ffmpeg-concat>>
- <<link|GitHub: deps-opengl-raub - Binaries and headers for OpenGL-dependent...|https://github.com/node-3d/deps-opengl-raub>>
- <<link|GitHub: ffmpeg-concat - Concats a list of videos together using ffmpeg with...|https://github.com/transitive-bullshit/ffmpeg-concat>>
- <<link|GitHub: glfw-raub - GLFW for Node.js (from node-3d)|https://github.com/node-3d/glfw-raub>>
- <<link|GitHub: glx.h - No such file or directory - Google Search (from GL)|https://www.google.com/search?client=safari&rls=en&q=GL/glx.h:+No+such+file+or+directory&ie=UTF-8&oe=UTF-8>>
- <<link|GitHub: headless-gl - 🎃 Windowless WebGL for node.js (from stackgl)|https://github.com/stackgl/headless-gl#supported-platforms-and-nodejs-versions>>
- <<link|GitHub: headless-gl - 🎃 Windowless WebGL for node.js (from stackgl)|https://github.com/stackgl/headless-gl>>
- <<link|GitHub: headless-gl - 🎃 Windowless WebGL for node.js (from stackgl)|https://github.com/stackgl/headless-gl#ubuntudebian>>
- <<link|GitHub: headless-gl - 🎃 Windowless WebGL for node.js (from stackgl)|https://github.com/stackgl/headless-gl#how-should-i-set-up-a-development-environment-for-headless-gl>>
- <<link|GitHub: headless-gl - 🎃 Windowless WebGL for node.js (from stackgl)|https://github.com/stackgl/headless-gl#system-dependencies>>
- <<link|GitHub: mukaGL - Toy implementation of OpenGL (from mukadr)|https://github.com/mukadr/mukaGL>>
- <<link|GitHub: node-glfw - nodejs bindings to GLFW (from mikeseven)|https://github.com/mikeseven/node-glfw>>
- <<link|GitHub: node-gyp - Node.js native addon build tool (from nodejs)|https://github.com/nodejs/node-gyp>>
- <<link|GitHub: node-gyp - Node.js native addon build tool (from nodejs)|https://github.com/nodejs/node-gyp#installation>>
- <<link|GitHub: require-rebuild - Patch ``require()`` to recompile a node module if it...|https://github.com/juliangruber/require-rebuild>>
- <<link|GitHub: sharp - High performance Node.js image processing, the fastest module...|https://github.com/lovell/sharp>>
- <<link|gl - npm|https://www.npmjs.com/package/gl?activeTab=readme>>
- <<link|gl - npm|https://www.npmjs.com/package/gl/v/3.0.5>>
- <<link|GLEW: The OpenGL Extension Wrangler Library|http://glew.sourceforge.net/>>
- <<link|GLFW - An OpenGL library|https://www.glfw.org/>>
- <<link|Google Search: "ffmpeg-concat" repository ubuntu|https://www.google.com/search?newwindow=1&client=safari&rls=en&ei=0yyrX-S0AcSr5wL3yJiABQ&q=%22ffmpeg-concat%22+repository+ubuntu&oq=%22ffmpeg-concat%22+repository+ubuntu&gs_lcp=CgZwc3ktYWIQAzoECAAQR1DST1isWGCDWmgAcAJ4AIABbogB2QGSAQMwLjKYAQCgAQGqAQdnd3Mtd2l6yAEIwAEB&sclient=psy-ab&ved=0ahUKEwik6tvlmvnsAhXE1VkKHXckBlAQ4dUDCAw&uact=5>>
- <<link|Google Search: a software implementation of OpenGL ubuntu|https://www.google.com/search?newwindow=1&client=safari&rls=en&ei=57GqX8iMHsLy5gLXqIxo&q=a+software+implementation+of+OpenGL+ubuntu+&oq=a+software+implementation+of+OpenGL+ubuntu+&gs_lcp=CgZwc3ktYWIQAzIFCCEQoAEyBQghEKABMgUIIRCgAToECAAQRzoICCEQFhAdEB46BQghEKsCOgcIIRAKEKABUMdVWIlhYNZjaABwBHgAgAHTAYgBsQmSAQUxLjYuMZgBAKABAaoBB2d3cy13aXrIAQjAAQE&sclient=psy-ab&ved=0ahUKEwiInOrIpfjsAhVCuVkKHVcUAw0Q4dUDCAw&uact=5>>
- <<link|Google Search: anttweakbar|https://www.google.com/search?client=safari&rls=en&q=anttweakbar&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: apt get upgrate|https://www.google.com/search?client=safari&rls=en&q=apt+get+upgrate&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: arch linux|https://www.google.com/search?client=safari&rls=en&q=arch+linux&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: assemble video with transiation command line|https://www.google.com/search?newwindow=1&client=safari&rls=en&ei=lDOrX53CCpL65gKU1YyAAw&q=assemble+video+with+transiation+command+line+&oq=assemble+video+with+transiation+command+line+&gs_lcp=CgZwc3ktYWIQAzIHCCEQChCgATIHCCEQChCgATIHCCEQChCgATIHCCEQChCgAToECAAQRzoFCCEQqwI6CAghEBYQHRAeOgcIIRAKEKsCUPUKWM4VYKcXaABwAngAgAGqAYgBzw6SAQQxLjEzmAEAoAEBqgEHZ3dzLXdpesgBCMABAQ&sclient=psy-ab&ved=0ahUKEwid3J6eofnsAhUSvVkKHZQqAzAQ4dUDCAw&uact=5>>
- <<link|Google Search: av_interleaved_write_frame(): Input/output error|https://www.google.com/search?client=safari&rls=en&q=av_interleaved_write_frame():+Input/output+error&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: aws ec2 ecu|https://www.google.com/search?client=safari&rls=en&q=aws+ec2+ecu&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: aws ec2 instance types|https://www.google.com/search?client=safari&rls=en&q=aws+ec2+instance+types&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: aws ec2 prices|https://www.google.com/search?client=safari&rls=en&q=aws+ec2+prices&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: aws efs|https://www.google.com/search?client=safari&rls=en&q=aws+efs&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: aws servcies|https://www.google.com/search?client=safari&rls=en&q=aws+servcies&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: bash run one file from anotehr|https://www.google.com/search?client=safari&rls=en&q=bash+run+one+file+from+anotehr&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: binding.gyp not found|https://www.google.com/search?client=safari&rls=en&q=binding.gyp+not+found&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: c4 c5 c6 ec2 difference|https://www.google.com/search?client=safari&rls=en&q=c4+c5+c6+ec2+difference&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: c5ad vs c5a|https://www.google.com/search?client=safari&rls=en&q=c5ad+vs+c5a&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: Call to 'pkg-config --libs-only-L --libs-only-other x11 xi...|https://www.google.com/search?client=safari&rls=en&q=Call+to+%27pkg-config+--libs-only-L+--libs-only-other+x11+xi+xext%27+returned+exit+status+127&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: cpan auto configure|https://www.google.com/search?client=safari&rls=en&q=cpan+auto+configure&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: cpan install command line|https://www.google.com/search?client=safari&rls=en&q=cpan+install+command+line&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ec2 install exiftool|https://www.google.com/search?client=safari&rls=en&q=ec2+install+exiftool&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ec2 install ffmpeg|https://www.google.com/search?client=safari&rls=en&q=ec2+install+ffmpeg&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ec2 install node|https://www.google.com/search?client=safari&rls=en&q=ec2+install+node&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ec2 libx11|https://www.google.com/search?newwindow=1&client=safari&rls=en&ei=AQiqX4OrF8ig5wKa0r2YBA&q=ec2+libx11&oq=ec2+libx11&gs_lcp=CgZwc3ktYWIQAzIFCCEQoAE6BAgAEEc6CAgAEMkDEJECOgUIABCRAjoECC4QQzoECAAQQzoCCAA6BwgAEMkDEAo6BAgAEAo6BggAEBYQHjoJCAAQyQMQFhAeOggIABAWEAoQHjoHCAAQyQMQDToECAAQDVC4Olj2TmCZUGgCcAJ4AIABbIgB6gWSAQM1LjOYAQCgAQGqAQdnd3Mtd2l6yAEIwAEB&sclient=psy-ab&ved=0ahUKEwjDr5zFg_fsAhVI0FkKHRppD0MQ4dUDCAw&uact=5>>
- <<link|Google Search: ec2 prices|https://www.google.com/search?client=safari&rls=en&q=ec2+prices&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ec2 x11 dev|https://www.google.com/search?client=safari&rls=en&q=ec2+x11+dev&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: execute ssh script remotely|https://www.google.com/search?client=safari&rls=en&q=execute+ssh+script+remotely&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: exiftool cpan|https://www.google.com/search?client=safari&rls=en&q=exiftool+cpan&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: exiftool install|https://www.google.com/search?client=safari&rls=en&q=exiftool+install&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: extract .xz file|https://www.google.com/search?client=safari&rls=en&q=extract+.xz+file&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: Failed to create OpenGL context ubuntu|https://www.google.com/search?newwindow=1&client=safari&rls=en&ei=STirX-OuNaOB5wLhxLWADQ&q=Failed+to+create+OpenGL+context+ubuntu&oq=Failed+to+create+OpenGL+context+ubuntu&gs_lcp=CgZwc3ktYWIQAzIJCAAQyQMQFhAeMgYIABAWEB4yBggAEBYQHjoECAAQRzoHCAAQyQMQQzoCCAA6BAgAEENQxDVYzUBgzkRoAHACeACAAbIBiAHABpIBAzEuNpgBAKABAaoBB2d3cy13aXrIAQjAAQE&sclient=psy-ab&ved=0ahUKEwij95TdpfnsAhWjwFkKHWFiDdAQ4dUDCAw&uact=5>>
- <<link|Google Search: Failed to create OpenGL context. ec2|https://www.google.com/search?newwindow=1&client=safari&rls=en&ei=ATyrX-ryB8Gy5gLBq4awDQ&q=Failed+to+create+OpenGL+context.++ec2+&oq=Failed+to+create+OpenGL+context.++ec2+&gs_lcp=CgZwc3ktYWIQAzIFCCEQoAE6BAgAEEc6CQgAEMkDEBYQHjoGCAAQFhAeUIBTWNZdYPhfaABwAngAgAFviAGRA5IBAzIuMpgBAKABAaoBB2d3cy13aXrIAQjAAQE&sclient=psy-ab&ved=0ahUKEwiq9-CiqfnsAhVBmVkKHcGVAdYQ4dUDCAw&uact=5>>
- <<link|Google Search: ffmpeg concat|https://www.google.com/search?client=safari&rls=en&q=ffmpeg+concat&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ffmpeg install suse|https://www.google.com/search?client=safari&rls=en&q=ffmpeg+install+suse&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ffmpeg suse install|https://www.google.com/search?client=safari&rls=en&q=ffmpeg+suse+install&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ffmpeg-concat|https://www.google.com/search?client=safari&rls=en&q=ffmpeg-concat&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ffmpeg-concat concat error Error: ENOTEMPTY: directory not empty, rmdir|https://www.google.com/search?client=safari&rls=en&q=ffmpeg-concat+concat+error+%5BError:+ENOTEMPTY:+directory+not+empty,+rmdir&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ffmpeg-concat Error: Could not locate the bindings file|https://www.google.com/search?client=safari&rls=en&q=ffmpeg-concat+Error:+Could+not+locate+the+bindings+file&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ffmpeg-concat ubuntu|https://www.google.com/search?client=safari&rls=en&q=ffmpeg-concat+ubuntu&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ffmpeg-concat/node_modules/bindings/bindings.js|https://www.google.com/search?client=safari&rls=en&q=ffmpeg-concat/node_modules/bindings/bindings.js&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: florida ocalla|https://www.google.com/search?client=safari&rls=en&q=florida+ocalla&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: fractal design define 7|https://www.google.com/search?newwindow=1&client=safari&rls=en&ei=7j6rX4-cOvKP5wLv9pToBg&q=fractal+design+define+7&oq=fractal+design+def&gs_lcp=CgZwc3ktYWIQAxgAMgUIABDJAzICCAAyBAgAEEMyAggAMgIIADICCAAyBAgAEEMyAggAMgIIADICCAA6BAgAEEdQhjlYmj9gy0ZoAHACeACAAYABiAGsA5IBAzEuM5gBAKABAaoBB2d3cy13aXrIAQjAAQE&sclient=psy-ab>>
- <<link|Google Search: gl/glx/FunctionsGLX.o Error|https://www.google.com/search?client=safari&rls=en&q=gl/glx/FunctionsGLX.o%5D+Error&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: gyp: binding.gyp not found|https://www.google.com/search?client=safari&rls=en&q=gyp:+binding.gyp+not+found&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: init 6|https://www.google.com/search?client=safari&rls=en&q=init+6&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: install cpan module from the command line|https://www.google.com/search?client=safari&rls=en&q=install+cpan+module+from+the+command+line&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: install cpan modules from the command line|https://www.google.com/search?client=safari&rls=en&q=install+cpan+modules+from+the+command+line&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: install development version of node module|https://www.google.com/search?client=safari&rls=en&q=install+development+version+of+node+module&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: install node|https://www.google.com/search?client=safari&rls=en&q=install+node&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: install npm|https://www.google.com/search?client=safari&rls=en&q=install+npm&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: install software on suse|https://www.google.com/search?client=safari&rls=en&q=install+software+on+suse&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: libgl1-mesa-swx11 ubuntu|https://www.google.com/search?client=safari&rls=en&q=libgl1-mesa-swx11+ubuntu&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: libxkbfile-dev ec2|https://www.google.com/search?client=safari&rls=en&q=libxkbfile-dev+ec2&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: linux install cpan|https://www.google.com/search?client=safari&rls=en&q=linux+install+cpan&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: lnstall libx11 dev on ec2|https://www.google.com/search?client=safari&rls=en&q=lnstall+libx11+dev+on+ec2&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: mac scp mount|https://www.google.com/search?client=safari&rls=en&q=mac+scp+mount&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ModuleNotFoundError: No module named 'cv2'|https://www.google.com/search?client=safari&rls=en&q=ModuleNotFoundError:+No+module+named+%27cv2%27&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: node gl no build directory|https://www.google.com/search?client=safari&rls=en&q=node+gl+no+build+directory&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: node glew|https://www.google.com/search?client=safari&rls=en&q=node+glew&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: node install suse|https://www.google.com/search?client=safari&rls=en&q=node+install+suse&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: node mesa gl|https://www.google.com/search?newwindow=1&client=safari&rls=en&ei=xrKqX_WAIuKN5wK4w56gBA&q=node+mesa+gl+&oq=node+mesa+gl+&gs_lcp=CgZwc3ktYWIQAzIFCCEQoAEyBQghEKsCOgQIABBHOgkIABDJAxAWEB5QuUdYvlBg9VJoAHACeACAAYEBiAG9A5IBAzEuM5gBAKABAaoBB2d3cy13aXrIAQPAAQE&sclient=psy-ab&ved=0ahUKEwj1-5izpvjsAhXixlkKHbihB0QQ4dUDCAw&uact=5>>
- <<link|Google Search: node-glfw|https://www.google.com/search?client=safari&rls=en&q=node-glfw&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: node_modules/gl|https://www.google.com/search?client=safari&rls=en&q=node_modules/gl&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: node_modules/gl can'd find bindings|https://www.google.com/search?client=safari&rls=en&q=node_modules/gl+can%27d+find+bindings&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: node_modules/gl/build/webgl.node|https://www.google.com/search?client=safari&rls=en&q=node_modules/gl/build/webgl.node&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: npm headless-gl .node|https://www.google.com/search?newwindow=1&client=safari&rls=en&ei=xhmrX8XOONCJ5wK_2ZeIAQ&q=npm+headless-gl+.node&oq=npm+headless-gl+.node&gs_lcp=CgZwc3ktYWIQA1DSPlj0Q2CbRWgAcAB4AIABggGIAZoEkgEDMy4ymAEAoAEBqgEHZ3dzLXdpesABAQ&sclient=psy-ab&ved=0ahUKEwjFyc7QiPnsAhXQxFkKHb_sBREQ4dUDCAw&uact=5>>
- <<link|Google Search: npm mesa opengl|https://www.google.com/search?newwindow=1&client=safari&rls=en&ei=yMyqX8W8O8v25gL9npeYBQ&q=npm+mesa+opengl&oq=npm+mesa+opengl&gs_lcp=CgZwc3ktYWIQAzoJCAAQyQMQFhAeOgUIIRCgAToFCCEQqwI6BwghEAoQoAFQ2C1Y0D5gx0FoAXAAeACAAaQBiAGkB5IBAzUuNJgBAKABAaoBB2d3cy13aXrAAQE&sclient=psy-ab&ved=0ahUKEwjFwJaav_jsAhVLu1kKHX3PBVMQ4dUDCAw&uact=5>>
- <<link|Google Search: npm WARN deprecated har-validator@5.1.5: this library is no...|https://www.google.com/search?client=safari&rls=en&q=npm+WARN+deprecated+har-validator@5.1.5:+this+library+is+no+longer+supported&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: Package x11 was not found in the pkg-config search path.|https://www.google.com/search?client=safari&rls=en&q=Package+x11+was+not+found+in+the+pkg-config+search+path.&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: Package xi was not found in the pkg-config search path.|https://www.google.com/search?client=safari&rls=en&q=Package+xi+was+not+found+in+the+pkg-config+search+path.&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: Perhaps you should add the directory containing `x11.pc'|https://www.google.com/search?client=safari&rls=en&q=Perhaps+you+should+add+the+directory+containing+%60x11.pc%27&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: perl -m cpan install|https://www.google.com/search?client=safari&rls=en&q=perl+-m+cpan+install&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: perl autosetup cpan|https://www.google.com/search?client=safari&rls=en&q=perl+autosetup+cpan&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: record terminal|https://www.google.com/search?client=safari&rls=en&q=record+terminal&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: root cpan config file path|https://www.google.com/search?newwindow=1&client=safari&rls=en&ei=EBCqX6GNOaPn5gLr84-wBA&q=root+cpan+config+file+path+&oq=root+cpan+config+file+path+&gs_lcp=CgZwc3ktYWIQAzIICCEQFhAdEB46BAgAEEc6BQghEKABOgUIIRCrAjoHCCEQChCgAVCyJFj2LGCBNGgAcAJ4AIABrQeIAfwVkgEHMi00LjYtMpgBAKABAaoBB2d3cy13aXrIAQjAAQE&sclient=psy-ab&ved=0ahUKEwih1Zmdi_fsAhWjs1kKHev5A0YQ4dUDCAw&uact=5>>
- <<link|Google Search: rsync port|https://www.google.com/search?client=safari&rls=en&q=rsync+port&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: run command from ssh|https://www.google.com/search?client=safari&rls=en&q=run+command+from+ssh&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: run xvfb|https://www.google.com/search?newwindow=1&client=safari&rls=en&ei=TDyrX8edLoKW5wKHyJfACw&q=run+xvfb&oq=run+xvfb&gs_lcp=CgZwc3ktYWIQAzICCAAyAggAMgIIADIECAAQHjIECAAQHjIECAAQHjIECAAQHjIECAAQHjIECAAQHjIECAAQHjoECAAQDToGCAAQDRAeUKIUWKIUYPwVaAFwAHgAgAGMAYgB2gGSAQMxLjGYAQCgAQGqAQdnd3Mtd2l6wAEB&sclient=psy-ab&ved=0ahUKEwjH8-jGqfnsAhUCy1kKHQfkBbgQ4dUDCAw&uact=5>>
- <<link|Google Search: scenedetect|https://www.google.com/search?client=safari&rls=en&q=scenedetect&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: security pen testing linux|https://www.google.com/search?client=safari&rls=en&q=security+pen+testing+linux&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: spring security|https://www.google.com/search?client=safari&rls=en&q=spring+security&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ssh send single command|https://www.google.com/search?client=safari&rls=en&q=ssh+send+single+command&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: suse aws name|https://www.google.com/search?client=safari&rls=en&q=suse+aws+name&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ubuntu Error: Could not locate the bindings file "gl"|https://www.google.com/search?newwindow=1&client=safari&rls=en&q=ubuntu+Error:+Could+not+locate+the+bindings+file+%22gl%22&sa=X&ved=2ahUKEwiswY33n_jsAhXu1FkKHQfEDMUQ5t4CMAN6BAgHEAo&biw=1440&bih=734>>
- <<link|Google Search: ubuntu ffmpeg|https://www.google.com/search?client=safari&rls=en&q=ubuntu+ffmpeg&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ubuntu gflw|https://www.google.com/search?client=safari&rls=en&q=ubuntu+gflw&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ubuntu glew|https://www.google.com/search?client=safari&rls=en&q=ubuntu+glew&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ubuntu install make|https://www.google.com/search?client=safari&rls=en&q=ubuntu+install+make&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ubuntu install node|https://www.google.com/search?client=safari&rls=en&q=ubuntu+install+node&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ubuntu install python2|https://www.google.com/search?client=safari&rls=en&q=ubuntu+install+python2&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ubuntu libuv|https://www.google.com/search?client=safari&rls=en&q=ubuntu+libuv&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ubuntu node 20 install|https://www.google.com/search?client=safari&rls=en&q=ubuntu+node+20+install&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ubuntu opengl|https://www.google.com/search?client=safari&rls=en&q=ubuntu+opengl&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ubuntu opengl without gpu|https://www.google.com/search?client=safari&rls=en&q=ubuntu+opengl+without+gpu&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ubuntu pango|https://www.google.com/search?client=safari&rls=en&q=ubuntu+pango&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ubuntu reintsall package|https://www.google.com/search?client=safari&rls=en&q=ubuntu+reintsall+package&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: ubuntu Xvfb|https://www.google.com/search?client=safari&rls=en&q=ubuntu+Xvfb&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: unbuffer ssh output|https://www.google.com/search?client=safari&rls=en&q=unbuffer+ssh+output&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: unzip .zip ubuntu|https://www.google.com/search?client=safari&rls=en&q=unzip+.zip+ubuntu&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: update har-validator|https://www.google.com/search?client=safari&rls=en&q=update+har-validator&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: x11 not found node|https://www.google.com/search?client=safari&rls=en&q=x11+not+found+node&ie=UTF-8&oe=UTF-8>>
- <<link|Google Search: yum change repo|https://www.google.com/search?client=safari&rls=en&q=yum+change+repo&ie=UTF-8&oe=UTF-8>>
- <<link|gyp: binding.gyp not found (cwd: C:\Users\xxxx) while trying to load...|https://github.com/nodejs/node-gyp/issues/1709>>
- <<link|har-validator - npm|https://www.npmjs.com/package/har-validator>>
- <<link|Home | The Mesa 3D Graphics Library|https://www.mesa3d.org/>>
- <<link|How to extract .xz files? - Shell - Linux Tips|https://linux-tips.com/t/how-to-extract-xz-files/265/2>>
- <<link|How to install a OpenGL environment on Ubuntu | by Vincent Nicopolsky | Medium|https://medium.com/@Plimsky/how-to-install-a-opengl-environment-on-ubuntu-e3918cf5ab6c>>
- <<link|How to Install and Use FFmpeg on Ubuntu 18.04 | Linuxize|https://linuxize.com/post/how-to-install-ffmpeg-on-ubuntu-18-04/>>
- <<link|How to Install and Use FFmpeg on Ubuntu 20.04 | Linuxize|https://linuxize.com/post/how-to-install-ffmpeg-on-ubuntu-20-04/>>
- <<link|How to install CPAN modules into ActivePerl | ActiveState|https://www.activestate.com/blog/how-install-cpan-modules-activeperl/>>
- <<link|How to install cpan on RHEL 8 / CentOS 8 - LinuxConfig.org|https://linuxconfig.org/how-to-install-cpan-on-redhat-8>>
- <<link|How to install FFMPEG on EC2 running Amazon Linux? | by Vivek Maskara | Medium|https://maskaravivek.medium.com/how-to-install-ffmpeg-on-ec2-running-amazon-linux-451e4a8e2694>>
- <<link|How to install Mesa 3D Graphics Library for Vufori... - PTC Community|https://community.ptc.com/t5/Vuforia-Studio/How-to-install-Mesa-3D-Graphics-Library-for-Vuforia-Experience/td-p/620317>>
- <<link|How To Install Node.js on Ubuntu 18.04 | DigitalOcean|https://www.digitalocean.com/community/tutorials/how-to-install-node-js-on-ubuntu-18-04>>
- <<link|How To Install Node.js on Ubuntu 20.04 | DigitalOcean|https://www.digitalocean.com/community/tutorials/how-to-install-node-js-on-ubuntu-20-04>>
- <<link|How to install OpenGL in Ubuntu | Linux|https://www.includehelp.com/linux/how-to-install-opengl-in-ubuntu-linux.aspx>>
- <<link|How to Install OpenGL on Ubuntu Linux|http://www.codebind.com/linux-tutorials/install-opengl-ubuntu-linux/>>
- <<link|How To Install Perl Modules On Linux - OSTechNix|https://ostechnix.com/how-to-install-perl-modules-on-linux/>>
- <<link|How to Install Python 2 on Ubuntu 20.04 - Vultr.com|https://www.vultr.com/docs/how-to-install-python-2-on-ubuntu-20-04>>
- <<link|How To Run Multiple SSH Command On Remote Machine And Exit Safely - nixCraft|https://www.cyberciti.biz/faq/linux-unix-osx-bsd-ssh-run-command-on-remote-machine-server/>>
- <<link|How To Run Your Tests Headlessly with Xvfb|http://elementalselenium.com/tips/38-headless>>
- <<link|How to Unzip a Zip File in Ubuntu From Command Line | PhoenixNAP KB|https://phoenixnap.com/kb/how-to-unzip-file-linux-ubuntu>>
- <<link|How to update to mesa drivers with OpenGL 4.5 support? : Crostini|https://www.reddit.com/r/Crostini/comments/heq0m0/how_to_update_to_mesa_drivers_with_opengl_45/>>
- <<link|How to use ffmpeg to combine multiple videos to one|https://ma.ttias.be/use-ffmpeg-combine-multiple-videos/>>
- <<link|How to use GUI apps on AWS linux server over SSH with X11 forwarding from...|https://medium.com/@potatowagon/how-to-use-gui-apps-on-aws-linux-server-over-ssh-with-x11-forwarding-from-windows-1e80cd9571a8>>
- <<link|How To Use Node.js Modules with npm and package.json | DigitalOcean|https://www.digitalocean.com/community/tutorials/how-to-use-node-js-modules-with-npm-and-package-json>>
- <<link|HOWTO: Install your own Perl modules|https://www.osc.edu/book/export/html/4520>>
- <<link|HowToInstallCpanModules < Support < Foswiki|https://foswiki.org/Support.HowToInstallCpanModules>>
- <<link|Image::ExifTool - metacpan.org|https://metacpan.org/pod/Image::ExifTool>>
- <<link|INSTALL - Build and Installation guide for perl 5. - metacpan.org|https://metacpan.org/pod/distribution/perl/INSTALL>>
- <<link|INSTALL - metacpan.org|https://metacpan.org/pod/distribution/Text-SenseClusters/doc/INSTALL.pod>>
- <<link|Install and manage Perl modules in my home directory on Carbonate at IU|https://kb.iu.edu/d/baiu>>
- <<link|Install CPAN on linux – Linux Administration Blog|http://www.initlinux.com/install-cpan-on-linux/>>
- <<link|Install ExifTool|https://docs.bitnami.com/aws/apps/resourcespace/configuration/install-exiftool/>>
- <<link|Install FFmpeg on openSUSE using the Snap Store | Snapcraft|https://snapcraft.io/install/ffmpeg/opensuse>>
- <<link|Install package openSUSE:Leap:15.1 / ffmpeg-4|https://software.opensuse.org/download/package?package=ffmpeg-4&project=openSUSE%3ALeap%3A15.1>>
- <<link|Install Python 2 on Ubuntu 20.04 Focal Fossa Linux - LinuxConfig.org|https://linuxconfig.org/install-python-2-on-ubuntu-20-04-focal-fossa-linux>>
- <<link|Installation - sharp - High performance Node.js image processing|https://sharp.pixelplumbing.com/install>>
- <<link|Installation - sharp - High performance Node.js image processing|https://sharp.pixelplumbing.com/install#common-problems>>
- <<link|Installation - sharp - High performance Node.js image processing|https://sharp.pixelplumbing.com/install#aws-lambda>>
- <<link|Installation - sharp - High performance Node.js image processing|https://sharp.pixelplumbing.com/install#building-from-source>>
- <<link|Installing a Perl Module from CPAN on Windows, Linux and Mac OSX|https://perlmaven.com/how-to-install-a-perl-module-from-cpan>>
- <<link|Installing CPAN from a non-root account|http://alumni.soe.ucsc.edu/~you/notes/perl-module-install.html>>
- <<link|Installing ExifTool|https://exiftool.org/install.html>>
- <<link|Installing Node.js via package manager | Node.js|https://nodejs.org/en/download/package-manager/>>
- <<link|Installing Node.js via package manager | Node.js|https://nodejs.org/en/download/package-manager/#debian-and-ubuntu-based-linux-distributions-enterprise-linux-fedora-and-snap-packages>>
- <<link|Installing Perl Modules - www.cpan.org|http://www.cpan.org/modules/INSTALL.html>>
- <<link|jogl - JOGL 2.0 (OpenGL/OpenGL-ES) backend for LibGDX | Page 13|http://forum.jogamp.org/JOGL-2-0-OpenGL-OpenGL-ES-backend-for-LibGDX-td4027689i240.html>>
- <<link|John Van Sickle - FFmpeg Static Builds|https://www.johnvansickle.com/ffmpeg/>>
- <<link|Kali Linux | Penetration Testing and Ethical Hacking Linux Distribution|https://www.kali.org/>>
- <<link|Keep getting ENOTEMPTY error. Unable to delete directories. · Issue #34 ·...|https://github.com/gruntjs/grunt-contrib-clean/issues/34>>
- <<link|Learn Perl - CPAN and Perl Configuration Howto|http://learnperl.scratchcomputing.com/tutorials/configuration/>>
- <<link|linux - What is the difference between reboot , init 6 and shutdown -r now? -...|https://unix.stackexchange.com/questions/64280/what-is-the-difference-between-reboot-init-6-and-shutdown-r-now>>
- <<link|MDN: Setting up a Node development environment - Learn web development|https://developer.mozilla.org/en-US/docs/Learn/Server-side/Express_Nodejs/development_environment>>
- <<link|MLT - Documentation|https://www.mltframework.org/docs/melt/>>
- <<link|Module::AutoInstall - Automatic install of dependencies via CPAN - metacpan.org|https://metacpan.org/pod/Module::AutoInstall>>
- <<link|Mountain Duck|https://mountainduck.io/>>
- <<link|node-glfw-3 - npm|https://www.npmjs.com/package/node-glfw-3>>
- <<link|node-gyp error in production · Issue #1259 · nodejs/node-gyp|https://github.com/nodejs/node-gyp/issues/1259>>
- <<link|node.js server rendering with node-webgl and/or headless-gl · Issue #7085 ·...|https://github.com/mrdoob/three.js/issues/7085>>
- <<link|Now Available, Amazon EC2 C5a instances featuring 2nd Generation AMD EPYC Processors|https://aws.amazon.com/about-aws/whats-new/2020/06/now-available-amazon-ec2-c5a-instances-featuring-2nd-generation-amd-epyc-processors/>>
- <<link|npm install tileserver-gl failed · Issue #325 · maptiler/tileserver-gl|https://github.com/maptiler/tileserver-gl/issues/325>>
- <<link|npm segmentation fault - Technical Issues and Assistance / Applications -...|https://archived.forum.manjaro.org/t/npm-segmentation-fault/117844>>
- <<link|npm | get npm|https://www.npmjs.com/get-npm>>
- <<link|npm-install | npm Docs|https://docs.npmjs.com/cli/v6/commands/npm-install>>
- <<link|openSUSE Software|https://software.opensuse.org/package/ffmpeg>>
- <<link|openSUSE Software|https://software.opensuse.org/package/ffmpeg-4>>
- <<link|Package - bindings|https://developer.aliyun.com/mirror/npm/package/bindings>>
- <<link|Package - deeplearn-gl|https://developer.aliyun.com/mirror/npm/package/deeplearn-gl/v/4.0.5>>
- <<link|Package - ffmpeg-concat|https://developer.aliyun.com/mirror/npm/package/ffmpeg-concat>>
- <<link|Package fails to install in some environments · Issue #71 · stackgl/headless-gl|https://github.com/stackgl/headless-gl/issues/71>>
- <<link|Package x11 not found in pkg-config search path|https://lists.freedesktop.org/archives/xorg/2006-May/015571.html>>
- <<link|Perl Modules - www.cpan.org|http://mirror.netcologne.de/cpan/modules/index.html>>
- <<link|PyPI: scenedetect|https://pypi.org/project/scenedetect/>>
- <<link|Question / Help - Failed to create OpenGL context on Ubuntu 14.04 | OBS Forums|https://obsproject.com/forum/threads/failed-to-create-opengl-context-on-ubuntu-14-04.25286/>>
- <<link|request and har-validator deprecated · Issue #18821 · angular/angular-cli|https://github.com/angular/angular-cli/issues/18821>>
- <<link|Run / Execute Command Using SSH - nixCraft|https://www.cyberciti.biz/faq/unix-linux-execute-command-using-ssh/>>
- <<link|Send Remote Commands Via SSH — Malcontent Comics Incorporated Presents:|https://malcontentcomics.com/systemsboy/2006/07/send-remote-commands-via-ssh.html>>
- <<link|Software implementation - OpenGL - Khronos Forums|https://community.khronos.org/t/software-implementation/50445>>
- <<link|software installation - How do I install make? - Ask Ubuntu|https://askubuntu.com/questions/161104/how-do-i-install-make>>
- <<link|Spring Security|https://spring.io/projects/spring-security>>
- <<link|SSH output isn't line buffered? - Unix & Linux Stack Exchange|https://unix.stackexchange.com/questions/21920/ssh-output-isnt-line-buffered>>
- <<link|Stack Overflow: amazon ec2 - EC2 Instance install x11 libraries|https://stackoverflow.com/questions/26634707/ec2-instance-install-x11-libraries>>
- <<link|Stack Overflow: Cannot find module cv2 when using OpenCV|https://stackoverflow.com/questions/19876079/cannot-find-module-cv2-when-using-opencv>>
- <<link|Stack Overflow: Error: Could not locate the bindings file. Tried: #56|https://stackoverflow.com/questions/27541586/error-could-not-locate-the-bindings-file-tried-56>>
- <<link|Stack Overflow: fatal error 'GL/glx.h' file not found on mac after Xquartz is installed|https://stackoverflow.com/questions/40186686/fatal-error-gl-glx-h-file-not-found-on-mac-after-xquartz-is-installed>>
- <<link|Stack Overflow: FFMpeg Error av_interleaved_write_frame():|https://stackoverflow.com/questions/2787539/ffmpeg-error-av-interleaved-write-frame>>
- <<link|Stack Overflow: GitHub: glx.h - No such file or directory libgl1-mesa-dev...|https://stackoverflow.com/questions/59339173/fatal-error-gl-glx-h-no-such-file-or-directory-libgl1-mesa-dev-already-install>>
- <<link|Stack Overflow: How do I automate CPAN configuration?|https://stackoverflow.com/questions/3462058/how-do-i-automate-cpan-configuration>>
- <<link|Stack Overflow: How to call one shell script from another shell script?|https://stackoverflow.com/questions/8352851/how-to-call-one-shell-script-from-another-shell-script>>
- <<link|Stack Overflow: How to install libuv on ubuntu?|https://stackoverflow.com/questions/42175630/how-to-install-libuv-on-ubuntu>>
- <<link|Stack Overflow: How to use SSH to run a local shell script on a remote machine?|https://stackoverflow.com/questions/305035/how-to-use-ssh-to-run-a-local-shell-script-on-a-remote-machine>>
- <<link|Stack Overflow: In C, how do I install GL/glx.h , GL/gl.h and GL/glu.h on Mac OS|https://stackoverflow.com/questions/62586763/in-c-how-do-i-install-gl-glx-h-gl-gl-h-and-gl-glu-h-on-mac-os>>
- <<link|Stack Overflow: Is it possible to specify a different ssh port when using rsync?|https://stackoverflow.com/questions/4549945/is-it-possible-to-specify-a-different-ssh-port-when-using-rsync>>
- <<link|Stack Overflow: node modules - Cannot install tileserver-gl using npm|https://stackoverflow.com/questions/59404426/cannot-install-tileserver-gl-using-npm>>
- <<link|Stack Overflow: node.js - heroku pkg-config install failure|https://stackoverflow.com/questions/27414127/heroku-pkg-config-install-failure/27586911>>
- <<link|Stack Overflow: node.js - Publish development version of NPM package|https://stackoverflow.com/questions/21355508/publish-development-version-of-npm-package>>
- <<link|Stack Overflow: OpenGL without a graphics card|https://stackoverflow.com/questions/7310885/opengl-without-a-graphics-card>>
- <<link|Stack Overflow: rsync port 22: Connection timed out|https://stackoverflow.com/questions/21574740/rsync-port-22-connection-timed-out>>
- <<link|symbol lookup error on nVidia · Issue #65 · stackgl/headless-gl|https://github.com/stackgl/headless-gl/issues/65>>
- <<link|The power of prediction markets : Nature News & Comment|https://www.nature.com/news/the-power-of-prediction-markets-1.20820>>
- <<link|Top 9 User-friendly Arch-Based Linux Distributions [2020]|https://itsfoss.com/arch-based-linux-distros/>>
- <<link|Tutorial: Setting Up Node.js on an Amazon EC2 Instance - AWS SDK for JavaScript|https://docs.aws.amazon.com/sdk-for-javascript/v2/developer-guide/setting-up-node-on-ec2-instance.html>>
- <<link|ubuntu - Force software based opengl rendering - Super User|https://superuser.com/questions/106056/force-software-based-opengl-rendering>>
- <<link|Ubuntu – Details of package libgl1-mesa-dev in bionic|https://packages.ubuntu.com/bionic/libgl1-mesa-dev>>
- <<link|Ubuntu – Details of package xvfb in xenial|https://packages.ubuntu.com/xenial/xvfb>>
- <<link|Ubuntu – Package Search Results -- glew|https://packages.ubuntu.com/search?keywords=glew>>
- <<link|Ubuntu – Package Search Results -- glfw|https://packages.ubuntu.com/search?keywords=glfw>>
- <<link|Ubuntu – Package Search Results -- libuv|https://packages.ubuntu.com/search?searchon=sourcenames&keywords=libuv>>
- <<link|Usage - asciinema|https://asciinema.org/docs/usage>>
- <<link|Using dynamic require on node targets WITHOUT resolve or bundle the target...|https://github.com/webpack/webpack/issues/4175#issuecomment-342931035>>
- <<link|Vagrant Guide — Ansible Documentation|https://docs.ansible.com/ansible/latest/scenario_guides/guide_vagrant.html>>
- <<link|webgl.node: undefined symbol:...|https://github.com/gpujs/gpu.js/issues/505>>
- <<link|What is Apt-get upgrade and dist-upgrade commands and how to use them – Linux Hint|https://linuxhint.com/apt_get_upgrade_dist_upgrade/>>
- <<link|Wikipedia: Ocala, Florida|https://en.wikipedia.org/wiki/Ocala,_Florida>>
- <<link|yarn fails on Ubuntu EC2 - x11 not found · Issue #72106 · microsoft/vscode|https://github.com/microsoft/vscode/issues/72106>>

-- /pre

-- metadata
-- created: 2020-11-10T00:00:00-04:00
-- id: 20enutos
-- status: published
-- type: post
-- SCRUBBED_NEO: false
-- tag: live-coding

"#;

        let config = SiteConfig::mock1_basic();
        // let whatever = do_parse(source, &config.sections);
        //dbg!(whatever);
        //assert_eq!(1, 0);
        assert!(do_parse(source, &config.sections).is_ok());
    }
}
