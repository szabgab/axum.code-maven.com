use strict;
use warnings;
use feature 'say';

use LWP::Simple qw(get);
use LWP::UserAgent ();

my $FILE = "src/ecosystem.md";

main();
exit;

my $total = 0;
my $skipped = 0;
my $failure = 0;
my $success = 0;
sub main {
    #my ($command) = @ARGV;
    #$command //= '';
    #if ($command eq "update") {
    #    download_file();
    #} elsif ($command eq 'check') {
        my $start = time;
        check_content();
        my $elapsed = int(time - $start);
        say "$success successes, $skipped skipped, and $failure failures out of a total of $total URLs. Elapsed time: $elapsed";
        if ($success + $skipped + $failure != $total) {
            say "ERROR: Something does not add up!";
        }
        #} else {
        #    die "Usage: $0 [update|check]\n"
        #}
}

sub download_file {
    my $source_url = "https://raw.githubusercontent.com/tokio-rs/axum/refs/heads/main/ECOSYSTEM.md";

    my $content = get($source_url);
    open my $fh, ">:utf8", $FILE or die;
    print $fh $content;
    close $fh;

    return $content;
}

sub read_ecosystem_file {
    open my $fh, "<:utf8", $FILE or die;
    local $/ = undef;
    my $content = <$fh>;
    return $content;
}

sub check_content {
    my $content = read_ecosystem_file();

    my @lines = split /\n/, $content;
    for my $line (@lines) {
        #say "XX: $line";
        if ($line =~ m{(http://[^\) ]+)}) {
            my $url = $1;
            say "ERROR: http-link in $line\n";
            check($url);
        }
        my @links = $line =~ m{(https://[^\) ]+)};
        for my $url (@links) {
            check($url);
        }
    }
}

sub check {
    my ($url) = @_;
    $total++;
    say "Checking `$url`";
    if ($url eq "https://carlosmv.hashnode.dev/getting-started-with-axum-rust") {
        say "This URL returns a '403 Forbidden' error for our script but worked when using a regular browser";
        $skipped++;
        return;
    }
    if ($url =~ m{https://crates.io/crates/}) {
        # these return 404 now. Dow I need to set the User Agent to make this work?
        # they also return 404 for curl even if I set the user:
        # curl -A "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:148.0) Gecko/20100101 Firefox/148.0" -i https://crates.io/crates/axum-msgpack
        # For now we accept these as working without checking.
        $skipped++;
        return;
    }

    my $result = get($url);
    my $ua = LWP::UserAgent->new(timeout => 10, agent => "Agent for https://axum.code-maven.com/");
    my $response = $ua->get($url);
    if ($response->is_success) {
        if (length($response->decoded_content) < 100) {
            $failure++;
            say "ERROR: Content from `$url` seems to be too short";
            say $response->decoded_content;
        } else {
            $success++;
        }
    } else {
        say "ERROR: Failed to download `$url` status: " . $response->status_line . "\n";
        $failure++;
    }
}
