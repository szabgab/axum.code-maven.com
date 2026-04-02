use strict;
use warnings;
use feature 'say';

use Cwd qw(cwd);
use Data::Dumper qw(Dumper);
use IPC::Run qw(run timeout);

use Test::More;

my $root = cwd();
say "Root $root";

my $globber = $ENV{EXAMPLES} // '';
my @examples = glob "src/examples/$globber*";
say Dumper \@examples;

for my $dir (@examples) {
    next if $dir eq "src/examples/meetup";
    next if $dir eq "src/examples/calculator-get";
    say $dir;
    chdir $dir;
    {
        my @cmd = ('cargo', 'test');
        my ($in, $out, $err);
        my $return = run \@cmd, \$in, \$out, \$err, timeout( 40 );
        my $exit_code = $?;
        is $return, 1, "return $dir";
        is $exit_code, 0, "exit code $dir";
    }
    {
        my @cmd = ('cargo', 'fmt', '--check');
        my ($in, $out, $err);
        my $return = run \@cmd, \$in, \$out, \$err, timeout( 40 );
        my $exit_code = $?;
        is $return, 1, "return $dir";
        is $exit_code, 0, "exit code $dir";
    }
    {
        my @cmd = ('cargo', 'clippy');
        my ($in, $out, $err);
        my $return = run \@cmd, \$in, \$out, \$err, timeout( 40 );
        my $exit_code = $?;
        is $return, 1, "return $dir";
        is $exit_code, 0, "exit code $dir";
    }



    chdir $root;
}

done_testing;
