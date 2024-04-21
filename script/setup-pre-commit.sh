mkdir -p ./.git/hooks
rm ./.git/hooks/pre-commit
cp ./script/pre-commit.sh ./.git/hooks/pre-commit
chmod +x ./.git/hooks/pre-commit