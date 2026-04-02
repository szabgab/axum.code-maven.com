use strict;
use warnings;
use feature 'say';

use Cwd qw(cwd);
use Data::Dumper qw(Dumper);
use IPC::Run qw(run timeout);

use Test::More;

my $root = cwd();
say "Root $root";

my @examples = glob "src/examples/*";
#say Dumper \@examples;

for my $dir (@examples) {
    say $dir;
    chdir $dir;
    my @cmd = ('cargo', 'test');
    my ($in, $out, $err);
    my $return = run \@cmd, \$in, \$out, \$err, timeout( 40 );
    is $return, 1, "return $dir";

    chdir $root;
}

done_testing;
